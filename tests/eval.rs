#![recursion_limit="128"]
#[macro_use]
extern crate fungi_lang;
extern crate adapton;

use std::rc::Rc;
use fungi_lang::ast::*;
use fungi_lang::eval;
use fungi_lang::vis;


fn eval_closed_exp(e:Exp) -> eval::ExpTerm {
    eval::eval(vec![], e)
}

fn eval_test_equiv(e1:Exp, e2:Exp) {
    println!("> {:?}\n\n", e1);
    let t1 = eval_closed_exp(e1);
    let t2 = eval_closed_exp(e2);
    // let (t1, x1) = capture_traces(move ||
    //     eval_closed_exp(vis::label_exp(e1, &mut 0))
    // );
    // let (t2, x2) = capture_traces(move ||
    //     eval_closed_exp(vis::label_exp(e2, &mut 0))
    // );
    // println!("Traces: {:?}\n\n", x1);
    assert_eq!(t1, t2)
}

#[test]
fn eval_force_anon_thunk () {
    eval_test_equiv(
        fgi_exp![
            let x = {ret 1}
            let y = {ret 2}
            let t = {ret thunk x + y}
            {force t}
        ],
        fgi_exp![
            ret 3
        ])        
}

#[test]
fn eval_let_pair_natlt () {
    eval_test_equiv(
        fgi_exp![
            let pair  = {ret (1, 2)}
            let (x,y) = {ret pair}
            x < y
        ],
        fgi_exp![
            ret true
        ])
}

#[test]
fn eval_lambda_app () {
    eval_test_equiv(
        fgi_exp![
            {#x.#y.x < y} 1 2
        ],
        fgi_exp![
            ret true
        ]);

    eval_test_equiv(
        fgi_exp![
            let x = {{#x.#y.x + y} 1 2}
            {#z1.#z2.#z3. if {z1 < z2} {ret 123} else {ret 321}}
            x 4 666
        ],
        fgi_exp![
            ret 123
        ]);
}

#[test]
fn eval_case () {
    eval_test_equiv(
        fgi_exp![
            match (inj1 2) {
                x => {x + 1}
                y => {ret 0}
            }
        ],
        fgi_exp![
            ret 3
        ]);
    
    eval_test_equiv(
        fgi_exp![
            match (inj2 2) {
                x => {x + 1}
                y => {ret 0}
            }
        ],
        fgi_exp![
            ret 0
        ]);

    // test nested cases; nested injections are a little awkward
    eval_test_equiv(
        fgi_exp![
            match (inj2 (inj2 2)) {
                x => {ret 0}
                x => {ret 0}
                y => {ret y}
            }
        ],
        fgi_exp![
            ret 2
        ])
}

#[test]
fn eval_fix () {
    eval_test_equiv(
        fgi_exp![
            let rec f:(Thk[0] Nat -> (Nat -> (F Nat |> {0;0}) |> {0;0}) {0;0}) = {
                #x. if {x == 0} {ret 123} else {ret 321}
            }
            {force f} 0
        ],
        fgi_exp![
            ret 123
        ]);
        
    eval_test_equiv(
        fgi_exp![
            let rec f:(Thk[0] Nat -> (Nat -> (F Nat |> {0;0}) |> {0;0}) {0;0}) = {
                #x. if {x == 0} {{force f} 1} else {ret 321}
            }
            {force f} 0
        ],
        fgi_exp![
            ret 321
        ]);

    eval_test_equiv(
        fgi_exp![
            let rec f:(Thk[0] Nat -> (Nat -> (F Nat |> {0;0}) |> {0;0}) {0;0}) = {
                #x. if {x == 0} {let x = {x + 1} {force f} x} else {ret x}
            }
            {force f} 0
        ],
        fgi_exp![
            ret 1
        ]);

    eval_test_equiv(
        fgi_exp![
            let rec f:(Thk[0] Nat -> (Nat -> (F Nat |> {0;0}) |> {0;0}) {0;0}) = {
                #x. #end. if {x < end} {let x = {x + 1} {force f} x end} else {ret x}
            }
            {force f} 0 2
        ],
        fgi_exp![
            ret 2
        ])      

}

#[test]
fn trace_simple() {
    let exp = fgi_exp![
        let pair  = {ret (1, 2)}
        let (x,y) = {ret pair}
        x < y
    ];
    
    let vis_exp = vis::label_exp(exp, &mut 0);
    println!("Exp: {:?}\n\n", vis_exp);
    
    let (_term, traces) = vis::capture_traces(move || eval_closed_exp(vis_exp));
    println!("Traces: {:?}\n\n", traces);
    
    assert_eq!(traces.len(), 6);
}