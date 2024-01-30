use std::{
    fs::File,
    io::Write,
    process::Command,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use regex::Regex;
use tempfile::tempdir;

#[derive(Debug)]
pub struct SolverHandler {
    receiver: mpsc::Receiver<Result<(Duration, [[Option<u8>; 9]; 9]), SolverError>>,
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

#[derive(Debug)]
pub enum SolverError {
    Infeasible,
}

impl SolverHandler {
    pub fn new(puzzel: [[Option<u8>; 9]; 9]) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            thread::spawn(move || {
                let now = Instant::now();
                let result = solve(&puzzel).expect("meow?");

                sender.send(result.map(|sol| (now.elapsed(), sol))).expect("");
            })
        };
        return Self { receiver, handler };
    }

    pub fn try_get(&self) -> Result<Result<(Duration, [[Option<u8>; 9]; 9]), SolverError>> {
        Ok(self.receiver.try_recv()?)
    }
}

fn solve(puzzel: &[[Option<u8>; 9]; 9]) -> Result<Result<[[Option<u8>; 9]; 9], SolverError>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("model.lp");
    let mut model_file = File::create(file_path.clone())?;

    writeln!(model_file, "Maximize\n\t0")?;

    writeln!(model_file, "Subject To")?;
    // only one number can be assigned per cell
    for i in 0..9 {
        for j in 0..9 {
            writeln!(
                model_file,
                "{} = 1",
                (1..=9)
                    .map(|k| format!("x{}{}{}", i, j, k))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )?;
        }
    }

    // each number is exactly once in a row
    for i in 0..9 {
        for k in 1..=9 {
            writeln!(
                model_file,
                "{} = 1",
                (0..9)
                    .map(|j| format!("x{}{}{}", i, j, k))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )?;
        }
    }

    // each number is exactly once in a column
    for j in 0..9 {
        for k in 1..=9 {
            writeln!(
                model_file,
                "{} = 1",
                (0..9)
                    .map(|i| format!("x{}{}{}", i, j, k))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )?;
        }
    }

    // each 3x3 square must have all numbers
    for i in 0..3 {
        for j in 0..3 {
            for k in 1..=9 {
                writeln!(
                    model_file,
                    "{} = 1",
                    (0..3)
                        .flat_map(|x| (0..3).map(move |y| format!(
                            "x{}{}{}",
                            3 * i + x,
                            3 * j + y,
                            k
                        )))
                        .collect::<Vec<_>>()
                        .join(" + ")
                )?;
            }
        }
    }

    // puzzel constraint
    for i in 0..9 {
        for j in 0..9 {
            if let Some(k) = puzzel[i][j] {
                writeln!(model_file, "x{}{}{} = 1", i, j, k)?;
            }
        }
    }

    // binary var
    writeln!(
        model_file,
        "BINARY\n\t{}",
        (0..9)
            .flat_map(
                |i| (0..9).flat_map(move |j| (1..=9).map(move |k| format!("x{}{}{}", i, j, k)))
            )
            .collect::<Vec<_>>()
            .join(" ")
    )?;

    writeln!(model_file, "END")?;

    let output = Command::new(r"***REMOVED***")
        .arg("-f")
        .arg(file_path)
        .output()?;
    if !output.status.success() {
        println!("Failed: {}", String::from_utf8(output.stderr)?);
    }
    let output = String::from_utf8(output.stdout)?;

    drop(model_file);
    dir.close()?;

    return parse_scip_output(output);
}

fn parse_scip_output(output: String) -> Result<Result<[[Option<u8>; 9]; 9], SolverError>> {
    let re = Regex::new(r"([^=]+)============([^=]+)=============([^=]+)=================================([^x]*)(?<sol>[^=S]*)Statistics([\s\S]+)$").unwrap();

    let sol = if let Some(cap) = re.captures(&output) {
        cap.name("sol").unwrap().as_str()
    } else {
        return Ok(Err(SolverError::Infeasible));
    };
    if sol.is_empty() {
        return Ok(Err(SolverError::Infeasible));
    }
    let elements: Vec<&str> = sol.trim().split_whitespace().collect();
    let pos: Vec<_> = elements
        .chunks(3)
        .map(|c| {
            (
                c[0].chars().nth(1),
                c[0].chars().nth(2),
                c[0].chars().nth(3),
            )
        })
        .collect();

    let mut solution = [[None; 9]; 9];
    for (i, j, k) in pos.iter() {
        let i: usize = i.unwrap().to_digit(10).unwrap() as usize;
        let j: usize = j.unwrap().to_digit(10).unwrap() as usize;
        let k: u8 = k.unwrap().to_digit(10).unwrap() as u8;
        solution[i][j] = Some(k);
    }
    return Ok(Ok(solution));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let output = r#"
SCIP version 8.0.4 [precision: 8 byte] [memory: block] [mode: optimized] [LP solver: Soplex 6.0.4] [GitHash: a8e51afd1e]Copyright (c) 2002-2023 Zuse Institute Berlin (ZIB)

External libraries:
  Soplex 6.0.4         Linear Programming Solver developed at Zuse Institute Berlin (soplex.zib.de) [GitHash: 950b1658]
  CppAD 20180000.0     Algorithmic Differentiation of C++ algorithms developed by B. Bell (github.com/coin-or/CppAD)
  MPIR 3.0.0           Multiple Precision Integers and Rationals Library developed by W. Hart (mpir.org)
  ZIMPL 3.5.3          Zuse Institute Mathematical Programming Language developed by T. Koch (zimpl.zib.de)
  AMPL/MP 4e2d45c4     AMPL .nl file reader library (github.com/ampl/mp)
  PaPILO 2.1.3         parallel presolve for integer and linear optimization (github.com/scipopt/papilo) [GitHash: cec22d9]
  bliss 0.77           Computing Graph Automorphism Groups by T. Junttila and P. Kaski (www.tcs.hut.fi/Software/bliss/)
  Ipopt 3.12.9         Interior Point Optimizer developed by A. Waechter et.al. (github.com/coin-or/Ipopt)

user parameter file <scip.set> not found - using default parameters

read problem <C:\Users\Newspeak\Desktop\rust\sudoku\foo.lp>
============

original problem has 729 variables (729 bin, 0 int, 0 impl, 0 cont) and 350 constraints

solve problem
=============

presolving:
(round 1, fast)       590 del vars, 26 del conss, 0 add conss, 26 chg bounds, 0 chg sides, 0 chg coeffs, 0 upgd conss, 0 impls, 163 clqs
(round 2, fast)       1293 del vars, 231 del conss, 0 add conss, 28 chg bounds, 0 chg sides, 0 chg coeffs, 0 upgd conss, 0 impls, 0 clqs
presolving (3 rounds: 3 fast, 1 medium, 1 exhaustive):
 1389 deleted vars, 350 deleted constraints, 0 added constraints, 28 tightened bounds, 0 added holes, 0 changed sides, 0 changed coefficients
 0 implications, 0 cliques
transformed 1/1 original solutions to the transformed problem space
Presolving Time: 0.00

SCIP Status        : problem is solved [optimal solution found]
Solving Time (sec) : 0.00
Solving Nodes      : 0
Primal Bound       : +0.00000000000000e+00 (1 solutions)
Dual Bound         : +0.00000000000000e+00
Gap                : 0.00 %

primal solution (original space):
=================================

objective value:                                    0
x002                                                1   (obj:0)
x013                                                1   (obj:0)
x029                                                1   (obj:0)
x037                                                1   (obj:0)
x045                                                1   (obj:0)
x054                                                1   (obj:0)
x066                                                1   (obj:0)
x071                                                1   (obj:0)
x088                                                1   (obj:0)
x107                                                1   (obj:0)
x116                                                1   (obj:0)
x128                                                1   (obj:0)
x132                                                1   (obj:0)
x143                                                1   (obj:0)
x151                                                1   (obj:0)
x165                                                1   (obj:0)
x179                                                1   (obj:0)
x184                                                1   (obj:0)
x204                                                1   (obj:0)
x211                                                1   (obj:0)
x225                                                1   (obj:0)
x236                                                1   (obj:0)
x249                                                1   (obj:0)
x258                                                1   (obj:0)
x267                                                1   (obj:0)
x272                                                1   (obj:0)
x283                                                1   (obj:0)
x306                                                1   (obj:0)
x315                                                1   (obj:0)
x323                                                1   (obj:0)
x338                                                1   (obj:0)
x344                                                1   (obj:0)
x359                                                1   (obj:0)
x362                                                1   (obj:0)
x377                                                1   (obj:0)
x381                                                1   (obj:0)
x408                                                1   (obj:0)
x419                                                1   (obj:0)
x427                                                1   (obj:0)
x435                                                1   (obj:0)
x441                                                1   (obj:0)
x452                                                1   (obj:0)
x463                                                1   (obj:0)
x474                                                1   (obj:0)
x486                                                1   (obj:0)
x501                                                1   (obj:0)
x514                                                1   (obj:0)
x522                                                1   (obj:0)
x533                                                1   (obj:0)
x547                                                1   (obj:0)
x556                                                1   (obj:0)
x568                                                1   (obj:0)
x575                                                1   (obj:0)
x589                                                1   (obj:0)
x609                                                1   (obj:0)
x618                                                1   (obj:0)
x626                                                1   (obj:0)
x631                                                1   (obj:0)
x642                                                1   (obj:0)
x655                                                1   (obj:0)
x664                                                1   (obj:0)
x673                                                1   (obj:0)
x687                                                1   (obj:0)
x703                                                1   (obj:0)
x712                                                1   (obj:0)
x724                                                1   (obj:0)
x739                                                1   (obj:0)
x746                                                1   (obj:0)
x757                                                1   (obj:0)
x761                                                1   (obj:0)
x778                                                1   (obj:0)
x785                                                1   (obj:0)
x805                                                1   (obj:0)
x817                                                1   (obj:0)
x821                                                1   (obj:0)
x834                                                1   (obj:0)
x848                                                1   (obj:0)
x853                                                1   (obj:0)
x869                                                1   (obj:0)
x876                                                1   (obj:0)
x882                                                1   (obj:0)

Statistics
==========

SCIP Status        : problem is solved [optimal solution found]
Total Time         :       0.00
  solving          :       0.00
  presolving       :       0.00 (included in solving)
  reading          :       0.00
  copying          :       0.00 (0 times copied the problem)
Original Problem   :
  Problem name     : C:\Users\Newspeak\Desktop\rust\sudoku\foo.lp
  Variables        : 729 (729 binary, 0 integer, 0 implicit integer, 0 continuous)
  Constraints      : 350 initial, 350 maximal
  Objective        : maximize, 0 non-zeros (abs.min = 1e+20, abs.max = -1e+20)
Presolved Problem  :
  Problem name     : t_C:\Users\Newspeak\Desktop\rust\sudoku\foo.lp
  Variables        : 0 (0 binary, 0 integer, 0 implicit integer, 0 continuous)
  Constraints      : 0 initial, 0 maximal
  Objective        : minimize, 0 non-zeros (abs.min = 1e+20, abs.max = -1e+20)
  Nonzeros         : 0 constraint, 0 clique table
Presolvers         :   ExecTime  SetupTime  Calls  FixedVars   AggrVars   ChgTypes  ChgBounds   AddHoles    DelCons    AddCons   ChgSides   ChgCoefs
  boundshift       :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  convertinttobin  :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  domcol           :       0.00       0.00      1          0          0          0          0          0          0          0          0          0
  dualagg          :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  dualcomp         :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  dualinfer        :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  dualsparsify     :       0.00       0.00      1          0          0          0          0          0          0          0          0          0
  gateextraction   :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  implics          :       0.00       0.00      1          0          0          0          0          0          0          0          0          0
  inttobinary      :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  milp             :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  qpkktref         :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  redvub           :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  sparsify         :       0.00       0.00      1          0          0          0          0          0          0          0          0          0
  stuffing         :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  trivial          :       0.00       0.00      3        660          0          0          0          0          0          0          0          0
  tworowbnd        :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  dualfix          :       0.00       0.00      3          0          0          0          0          0          0          0          0          0
  genvbounds       :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  probing          :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  pseudoobj        :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  symmetry         :       0.00       0.00      1          0          0          0          0          0          0          0          0          0
  vbounds          :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  linear           :       0.00       0.00      3         28         41          0         28          0        350          0          0          0
  benders          :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  components       :       0.00       0.00      0          0          0          0          0          0          0          0          0          0
  root node        :          -          -      -          0          -          -          0          -          -          -          -          -
Constraints        :     Number  MaxNumber  #Separate #Propagate    #EnfoLP    #EnfoRelax  #EnfoPS    #Check   #ResProp    Cutoffs    DomReds       Cuts    Applied      Conss   Children
  benderslp        :          0          0          0          0          0          0          0          6          0          0          0          0          0          0          0
  integral         :          0          0          0          0          0          0          0          6          0          0          0          0          0          0          0
  benders          :          0          0          0          0          0          0          0          3          0          0          0          0          0          0          0
  countsols        :          0          0          0          0          0          0          0          3          0          0          0          0          0          0          0
  components       :          0          0          0          0          0          0          0          0          0          0          0          0          0          0          0
Constraint Timings :  TotalTime  SetupTime   Separate  Propagate     EnfoLP     EnfoPS     EnfoRelax   Check    ResProp    SB-Prop
  benderslp        :       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00
  integral         :       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00
  benders          :       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00
  countsols        :       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00
  components       :       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00       0.00
Propagators        : #Propagate   #ResProp    Cutoffs    DomReds
  dualfix          :          0          0          0          0
  genvbounds       :          0          0          0          0
  nlobbt           :          0          0          0          0
  obbt             :          0          0          0          0
  probing          :          0          0          0          0
  pseudoobj        :          0          0          0          0
  redcost          :          0          0          0          0
  rootredcost      :          0          0          0          0
  symmetry         :          0          0          0          0
  vbounds          :          0          0          0          0
Propagator Timings :  TotalTime  SetupTime   Presolve  Propagate    ResProp    SB-Prop
  dualfix          :       0.00       0.00       0.00       0.00       0.00       0.00
  genvbounds       :       0.00       0.00       0.00       0.00       0.00       0.00
  nlobbt           :       0.00       0.00       0.00       0.00       0.00       0.00
  obbt             :       0.00       0.00       0.00       0.00       0.00       0.00
  probing          :       0.00       0.00       0.00       0.00       0.00       0.00
  pseudoobj        :       0.00       0.00       0.00       0.00       0.00       0.00
  redcost          :       0.00       0.00       0.00       0.00       0.00       0.00
  rootredcost      :       0.00       0.00       0.00       0.00       0.00       0.00
  symmetry         :       0.00       0.00       0.00       0.00       0.00       0.00
  vbounds          :       0.00       0.00       0.00       0.00       0.00       0.00
Conflict Analysis  :       Time      Calls    Success    DomReds  Conflicts   Literals    Reconvs ReconvLits   Dualrays   Nonzeros   LP Iters   (pool size: [--,--])
  propagation      :       0.00          0          0          -          0        0.0          0        0.0          -          -          -
  infeasible LP    :       0.00          0          0          -          0        0.0          0        0.0          0        0.0          0
  bound exceed. LP :       0.00          0          0          -          0        0.0          0        0.0          0        0.0          0
  strong branching :       0.00          0          0          -          0        0.0          0        0.0          -          -          0
  pseudo solution  :       0.00          0          0          -          0        0.0          0        0.0          -          -          -
  applied globally :       0.00          -          -          0          0        0.0          -          -          0          -          -
  applied locally  :          -          -          -          0          0        0.0          -          -          0          -          -
Separators         :   ExecTime  SetupTime      Calls  RootCalls    Cutoffs    DomReds  FoundCuts ViaPoolAdd  DirectAdd    Applied ViaPoolApp  DirectApp      Conss
  cut pool         :       0.00          -          0          0          -          -          0          0          -          -          -          -          -    (maximal pool size:          0)
  aggregation      :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  > cmir           :          -          -          -          -          -          -          -          0          0          0          0          0          -
  > flowcover      :          -          -          -          -          -          -          -          0          0          0          0          0          -
  > knapsackcover  :          -          -          -          -          -          -          -          0          0          0          0          0          -
  cgmip            :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  clique           :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  closecuts        :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  convexproj       :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  disjunctive      :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  eccuts           :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  gauge            :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  gomory           :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  > gomorymi       :          -          -          -          -          -          -          -          0          0          0          0          0          -
  > strongcg       :          -          -          -          -          -          -          -          0          0          0          0          0          -
  impliedbounds    :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  interminor       :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  intobj           :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  mcf              :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  minor            :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  mixing           :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  oddcycle         :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  rapidlearning    :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  rlt              :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
  zerohalf         :       0.00       0.00          0          0          0          0          0          0          0          0          0          0          0
Cutselectors       :   ExecTime  SetupTime      Calls  RootCalls   Selected     Forced   Filtered  RootSelec   RootForc   RootFilt
  hybrid           :       0.00       0.00          0          0          0          0          0          0          0          0
Pricers            :   ExecTime  SetupTime      Calls       Vars
  problem variables:       0.00          -          0          0
Branching Rules    :   ExecTime  SetupTime   BranchLP  BranchExt   BranchPS    Cutoffs    DomReds       Cuts      Conss   Children
  allfullstrong    :       0.00       0.00          0          0          0          0          0          0          0          0
  cloud            :       0.00       0.00          0          0          0          0          0          0          0          0
  distribution     :       0.00       0.00          0          0          0          0          0          0          0          0
  fullstrong       :       0.00       0.00          0          0          0          0          0          0          0          0
  inference        :       0.00       0.00          0          0          0          0          0          0          0          0
  leastinf         :       0.00       0.00          0          0          0          0          0          0          0          0
  lookahead        :       0.00       0.00          0          0          0          0          0          0          0          0
  mostinf          :       0.00       0.00          0          0          0          0          0          0          0          0
  multaggr         :       0.00       0.00          0          0          0          0          0          0          0          0
  nodereopt        :       0.00       0.00          0          0          0          0          0          0          0          0
  pscost           :       0.00       0.00          0          0          0          0          0          0          0          0
  random           :       0.00       0.00          0          0          0          0          0          0          0          0
  relpscost        :       0.00       0.00          0          0          0          0          0          0          0          0
  vanillafullstrong:       0.00       0.00          0          0          0          0          0          0          0          0
Primal Heuristics  :   ExecTime  SetupTime      Calls      Found       Best
  LP solutions     :       0.00          -          -          0          0
  relax solutions  :       0.00          -          -          0          0
  pseudo solutions :       0.00          -          -          0          0
  strong branching :       0.00          -          -          0          0
  actconsdiving    :       0.00       0.00          0          0          0
  adaptivediving   :       0.00       0.00          0          0          0
  alns             :       0.00       0.00          0          0          0
  bound            :       0.00       0.00          0          0          0
  clique           :       0.00       0.00          0          0          0
  coefdiving       :       0.00       0.00          0          0          0
  completesol      :       0.00       0.00          0          0          0
  conflictdiving   :       0.00       0.00          0          0          0
  crossover        :       0.00       0.00          0          0          0
  dins             :       0.00       0.00          0          0          0
  distributiondivin:       0.00       0.00          0          0          0
  dps              :       0.00       0.00          0          0          0
  dualval          :       0.00       0.00          0          0          0
  farkasdiving     :       0.00       0.00          0          0          0
  feaspump         :       0.00       0.00          0          0          0
  fixandinfer      :       0.00       0.00          0          0          0
  fracdiving       :       0.00       0.00          0          0          0
  gins             :       0.00       0.00          0          0          0
  guideddiving     :       0.00       0.00          0          0          0
  indicator        :       0.00       0.00          0          0          0
  intdiving        :       0.00       0.00          0          0          0
  intshifting      :       0.00       0.00          0          0          0
  linesearchdiving :       0.00       0.00          0          0          0
  localbranching   :       0.00       0.00          0          0          0
  locks            :       0.00       0.00          0          0          0
  lpface           :       0.00       0.00          0          0          0
  mpec             :       0.00       0.00          0          0          0
  multistart       :       0.00       0.00          0          0          0
  mutation         :       0.00       0.00          0          0          0
  nlpdiving        :       0.00       0.00          0          0          0
  objpscostdiving  :       0.00       0.00          0          0          0
  octane           :       0.00       0.00          0          0          0
  ofins            :       0.00       0.00          0          0          0
  oneopt           :       0.00       0.00          0          0          0
  padm             :       0.00       0.00          0          0          0
  proximity        :       0.00       0.00          0          0          0
  pscostdiving     :       0.00       0.00          0          0          0
  randrounding     :       0.00       0.00          0          0          0
  rens             :       0.00       0.00          0          0          0
  reoptsols        :       0.00       0.00          0          0          0
  repair           :       0.00       0.00          0          0          0
  rins             :       0.00       0.00          0          0          0
  rootsoldiving    :       0.00       0.00          0          0          0
  rounding         :       0.00       0.00          0          0          0
  shiftandpropagate:       0.00       0.00          0          0          0
  shifting         :       0.00       0.00          0          0          0
  simplerounding   :       0.00       0.00          0          0          0
  subnlp           :       0.00       0.00          0          0          0
  trivial          :       0.00       0.00          1          0          0
  trivialnegation  :       0.00       0.00          0          0          0
  trustregion      :       0.00       0.00          0          0          0
  trysol           :       0.00       0.00          0          0          0
  twoopt           :       0.00       0.00          0          0          0
  undercover       :       0.00       0.00          0          0          0
  vbounds          :       0.00       0.00          0          0          0
  veclendiving     :       0.00       0.00          0          0          0
  zeroobj          :       0.00       0.00          0          0          0
  zirounding       :       0.00       0.00          0          0          0
  other solutions  :          -          -          -          0          -
LP                 :       Time      Calls Iterations  Iter/call   Iter/sec  Time-0-It Calls-0-It    ItLimit
  primal LP        :       0.00          0          0       0.00          -       0.00          0
  dual LP          :       0.00          0          0       0.00          -       0.00          0
  lex dual LP      :       0.00          0          0       0.00          -
  barrier LP       :       0.00          0          0       0.00          -       0.00          0
  resolve instable :       0.00          0          0       0.00          -
  diving/probing LP:       0.00          0          0       0.00          -
  strong branching :       0.00          0          0       0.00          -          -          -          0
    (at root node) :          -          0          0       0.00          -
  conflict analysis:       0.00          0          0       0.00          -
B&B Tree           :
  number of runs   :          1
  nodes            :          0 (0 internal, 0 leaves)
  feasible leaves  :          0
  infeas. leaves   :          0
  objective leaves :          0
  nodes (total)    :          0 (0 internal, 0 leaves)
  nodes left       :          0
  max depth        :         -1
  max depth (total):         -1
  backtracks       :          0 (0.0%)
  early backtracks :          0 (0.0%)
  nodes exc. ref.  :          0 (0.0%)
  delayed cutoffs  :          0
  repropagations   :          0 (0 domain reductions, 0 cutoffs)
  avg switch length:       0.00
  switching time   :       0.00
Root Node          :
  First LP value   :          -
  First LP Iters   :          0
  First LP Time    :       0.00
  Final Dual Bound :          -
  Final Root Iters :          0
  Root LP Estimate :                     -
Solution           :
  Solutions found  :          1 (1 improvements)
  First Solution   : +0.00000000000000e+00   (in run 1, after 0 nodes, 0.00 seconds, depth 0, found by <relaxation>)
  Gap First Sol.   :   infinite
  Gap Last Sol.    :   infinite
  Primal Bound     : +0.00000000000000e+00   (in run 1, after 0 nodes, 0.00 seconds, depth 0, found by <relaxation>)
  Dual Bound       : +0.00000000000000e+00
  Gap              :       0.00 %
Integrals          :      Total       Avg%
  primal-dual      :       0.00       0.00
  primal-ref       :       0.00       0.00
  dual-ref         :       0.00       0.00"#;

        let sol = parse_scip_output(output.to_string());
        println!("{:?}", sol);
    }

    #[test]
    fn test_solve() {
        let puzzel = vec![
            (0, 1, 3),
            (0, 2, 9),
            (1, 0, 7),
            (1, 2, 8),
            (1, 3, 2),
            (1, 8, 4),
            (2, 0, 4),
            (2, 3, 6),
            (2, 8, 3),
            (3, 3, 8),
            (3, 6, 2),
            (3, 7, 7),
            (4, 2, 7),
            (4, 6, 3),
            (5, 1, 4),
            (5, 2, 2),
            (5, 5, 6),
            (6, 0, 9),
            (6, 5, 5),
            (6, 8, 7),
            (7, 0, 3),
            (7, 5, 7),
            (7, 6, 1),
            (7, 8, 5),
            (8, 6, 9),
            (8, 7, 6),
        ];
        let mut sukoku = [[None; 9]; 9];
        for (i, j, k) in puzzel.into_iter() {
            sukoku[i][j] = Some(k);
        }
        let _ = solve(&sukoku).unwrap();
    }

    #[test]
    fn test_infeasible() {
        let puzzel = vec![(0, 0, 1), (0, 1, 1)];
        let mut sukoku = [[None; 9]; 9];
        for (i, j, k) in puzzel.into_iter() {
            sukoku[i][j] = Some(k);
        }
        let _ = solve(&sukoku).unwrap();
    }
}
