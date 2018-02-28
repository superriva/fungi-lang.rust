/*! Static (typing-time) term reduction/normalization. */

use std::fmt;
use std::rc::Rc;

use ast::*;

use subst;   
use bitype::{Ctx,Term};

pub fn is_normal_nmtm(ctx:&Ctx, n:&NameTm) -> bool {
    match *n {
        //
        // Forms that are normal; no reduction rules apply
        //
        NameTm::Var(_)     |
        NameTm::Name(_)    |
        NameTm::Lam(_,_,_) => true,
        //
        // Forms that are not normal
        //
        NameTm::Bin(_,_) |
        NameTm::App(_,_) => false,
        //
        // Other forms that we dont really need to consider:
        //
        NameTm::NoParse(_) => false,
        NameTm::WriteScope => false,
    }
}

/// XXX
/// Normalize name terms (expand definitions and reduce applications).
pub fn normal_nmtm(ctx:&Ctx, n:NameTm) -> NameTm {
    if is_normal_nmtm(ctx, &n) {
        n.clone()
    } else {
        let n_err = n.clone();
        match n {
            NameTm::Bin(n1,n2) => {
                let n1 = normal_nmtm_rec(ctx, n1);
                let n2 = normal_nmtm_rec(ctx, n2);
                match ((*n1).clone(),(*n2).clone()) {
                    (NameTm::Name(n1),
                     NameTm::Name(n2)) => {
                        // Normal form of `n`:
                        NameTm::Name(
                            Name::Bin(Rc::new(n1),
                                      Rc::new(n2)))
                    },
                    _ => {
                        // Fail: do nothing to `n`:
                        n_err
                    }
                }
            },
            NameTm::App(n1,n2) => {
                let n1 = normal_nmtm_rec(ctx, n1);
                let n2 = normal_nmtm_rec(ctx, n2);
                match ((*n1).clone(), (*n2).clone()) {
                    (NameTm::Lam(x,xg,n11), n2) => {
                        let n12 = subst::subst_nmtm_rec(n2, &x, n11);
                        normal_nmtm(ctx, (*n12).clone())
                    },
                    _ => {
                        // Fail: do nothing to `n`:
                        n_err
                    }
                }
            },
            // In all other cases (NoParse, etc), do nothing:
            n => n_err
        }
    }
}

pub fn normal_nmtm_rec(ctx:&Ctx, n:Rc<NameTm>) -> Rc<NameTm> {
    Rc::new(normal_nmtm(ctx, (*n).clone()))
}


/// Representation for "apart-normal" name set terms.
///
/// A _name set term_ is either a singleton name term `M`, or a
/// (disjoint) subset of the full set, represented by an index term
/// `i`.
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub enum NmSetTm {
    /// singleton name term `M`
    Single(NameTm),
    /// (disjoint) subset of the full set, represented by an index
    /// term `i`
    Subset(IdxTm),
}
pub type NmSetTms = Vec<NmSetTm>;

/// Index term "values", which _may_ be symbolic (viz., `Var` case).
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub enum IdxVal {
    /// Variables in values are not generally problematic, unless we
    /// need to use that value in an elimination form
    Var(Var),
    /// Compared with general index terms, name set terms consist of a more structured representation
    NmSet(NmSetTm),
    /// (Unique) unit value
    Unit,
    /// Pairs: both components are index term values
    Pair(IdxValRec, IdxValRec),
    /// Lambdas: same as general term form
    Lam(Var, Sort, IdxTmRec),
    /// No parse
    NoParse(String),
}
pub type IdxValRec = Rc<IdxVal>;


/// Index evaluation errors
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub enum IdxEvalErr {
    /// In some situations, the value to be eliminated (pair, function
    /// or set) is abstract/unknown, and thus, the elimination form
    /// cannot reduce.
    AbsIntroForm(Var),
    /// When the elimination form and intro forms do not agree on sort
    SortError,
}

/// Convert the (more restrictive) index term _value_ syntax back into
/// the (less restrictive) index term syntax.
pub fn idxtm_of_idxval(i:&IdxVal) -> IdxTm {
    panic!("XXX");    
}

/// If the term evaluates, result is an `IdxVal`; otherwise, the
/// result gives the (general) reason that evaluation cannot proceed.
pub fn idxtm_eval(ctx:&Ctx, i:IdxTm) -> Result<IdxVal,IdxEvalErr> {
    panic!("XXX");
}


/// Normalize index terms, by expanding definitions and reducing
/// function applications where possible.
///
///
/// # Example:
///
/// ```text
/// // Unknowns:
/// idxtm X : NmSet 
/// idxtm Y : NmSet
/// nmtm  z : Nm
/// 
/// idxtm bin     : Nm -> NmSet = (#x:Nm. {x * @1} % {x * @2})
/// idxtm set     : NmSet       = (({@3}%Y)%(X%{z}))
/// idxtm example : NmSet       = (bin) set
/// ```
///
/// The `example` term normalizes to the following term
///
/// ```text
/// {@3*@1} % {@3*@2} 
///   % (
///   ((#x:Nm. {x * @1} % {x * @2}) (X))
///   % (
///   {z*@1} % {z*@2}
///   %  (
///   ((#x:Nm. {x * @1} % {x * @2}) (Y))
///   %
///   0 )))
///  : NmSet
/// ```
/// 
/// Notice that the nested binary tree of disjoint unions (`%`) is
/// flattened into a list, where disjoint union (`%`) plays the rule
/// of `Cons`, and where empty set (`0`) plays the role of `Nil`.
///
/// Further, the flat-mapped function (`bin`) has been applied to the
/// set structure:
///
/// 1. The mapping function has been applied to the singleton sets of
/// name constant `@3` and name variable `z`, respectively.
///
/// 2. Meanwhile, as the set variables `X` and `Y` stand for unknown
/// _sets_ of names, the flat map is not reduced over these (unknown)
/// sets, but only distributed across the disjoint union (`%`) that
/// connects them.
///
/// ??? -- Do we need to implement symbolic set subtraction over this
/// final normalized structure ???  (It seems that's what we need to
/// implement the effect-checking logic of the `let` checking rule.)
///
pub fn normal_idxtm(ctx:&Ctx, i:&IdxTm) -> IdxTm {
    //let tms = nmsettms_of_idxtm(ctx, i);
    //return idxtm_of_nmsettms(ctx, &tms);
    panic!("TODO")
}

/// Convert the highly-structured, vectorized name set representation
/// into a less structured, AST representation.
pub fn idxtm_of_nmsettms(tms:&NmSetTms) -> IdxTm {
    let mut i : IdxTm = IdxTm::Empty;
    for t in tms.iter() {
        i = IdxTm::Apart(Rc::new(
            {
                match (*t).clone() {
                    NmSetTm::Single(m) => IdxTm::Sing(m),
                    NmSetTm::Subset(i) => i.clone()
                }
            }),
                         Rc::new(i)
        );        
    }
    return i
}

/// Normalize the index term with respect to name set apartness;
/// decomposes the index term across disjoint set unions.
pub fn nmsettms_of_idxtm(ctx:&Ctx, i:&IdxTm) -> NmSetTms {
    // Helper function
    fn nmsettms_rec(ctx:&Ctx, i:&IdxTm, out:&mut NmSetTms) {
        // XXX/TODO
    };    
    /// XXX/TODO
    let mut out = vec![];
    nmsettms_rec(ctx, i, &mut out);
    return out
}


/// Normalize types (expand definitions and reduce applications).
///
/// To normalize types, we generally need to **expand definitions** of
/// user-defined types, and **apply them** to type or index arguments.
///
/// ### Example:
///
/// Suppose the user defines `NmOp := foralli X:NmSet. 1 + Nm[X]` in
/// the context.  Then, `NmOp [{@1}]` normalizes to `1 + Nm[{@1}]`, by
/// using the body of the definition of `NmOp`, and reducing the
/// type-index application.
///
/// ### Reducible forms (not head normal form)
///
/// The following type forms are **reducible**:
///
///   1. `user(_)` / `Ident(_)`   -- user-defined identifiers (each reduces to its definition)
///   2. `(foralli a:g. A) [i]`   -- type-index application
///   3. `(forallt a:K. A) B`     -- type-type application
///
/// ### Normal forms (irreducible forms)
///
/// The following forms are "normal" (irreducible); they each have
/// intro/elim forms in the core language's program syntax:
///
///  1. Base types, sums, products
///  3. `Ref`, `Thk`, `Nm`, `(Nm->Nm)[_]`,
///  4. `exists`
///  5. `forallt`, `foralli`
///  6. `rec`
///  7. type variables, as introduced by `forallt` and `rec` (note: not the same as user-defined type names, which each have a known definition)
///  8. type applications in head normal form.
/// 
pub fn normal_type(ctx:&Ctx, typ:&Type) -> Type {
    match typ {
        // normal forms:
        &Type::Unit         |
        &Type::Var(_)       |
        &Type::Sum(_, _)    |
        &Type::Prod(_, _)   |
        &Type::Thk(_, _)    |
        &Type::Ref(_, _)    |
        &Type::Rec(_, _)    |
        &Type::Nm(_)        |
        &Type::NmFn(_)      |
        &Type::TypeFn(_,_,_)|
        &Type::IdxFn(_,_,_) |
        &Type::NoParse(_)   |
        &Type::Exists(_,_,_,_)
            =>
            typ.clone(),

        &Type::Ident(ref ident) => { match ident.as_str() {
            // Built-in primitives are normal; they lack a definition in the context:
            "Nat" | "Bool" | "String"
                => { typ.clone() }
            
            // all other identifiers are for defined types; look up the definition
            _ => { match ctx.lookup_type_def(ident) {
                Some(a) => normal_type(ctx, &a),
                _ => {
                    println!("undefined type: {} in\n{:?}", ident, ctx);
                    // Give up:
                    typ.clone()
                }
            }}
        }}
        &Type::TypeApp(ref a, ref b) => {
            let a = normal_type(ctx, a);
            let a = match a {
                Type::Rec(_,_) => unroll_type(&a),
                _ => a,
            };
            let b = normal_type(ctx, b);
            match a {
                Type::TypeFn(ref x, ref _k, ref body) => {
                    let body = subst::subst_type_type(b,x,(**body).clone());
                    normal_type(ctx, &body)
                },
                a => {
                    panic!("sort error: expected TypeFn, not {:?}", a);
                    typ.clone()
                }
            }
        }
        &Type::IdxApp(ref a, ref i) => {
            let a = normal_type(ctx, a);
            let a = match a {
                Type::Rec(_,_) => unroll_type(&a),
                _ => a,
            };
            match a {
                Type::IdxFn(ref x, ref _g, ref body) => {
                    let body = subst::subst_idxtm_type(i.clone(),x,(**body).clone());
                    normal_type(ctx, &body)
                },
                a => {
                    panic!("sort error: expected TypeFn, not {:?}", a);
                    typ.clone()
                }
            }
        }
    }
}

/*

Not head normal:
(#a. (#b. b) 3) 4
-->
(#a. 3) 4
-->
3 4
-/->

Not in normal form: (#b.b) 3) --> 3
(#x. ((#b.b) 3))

Is head normal (with lambda as outside thing)
(#x. ((#b.b) 3))

Head normal (with application as outside thing)
x 1 2 3
^
| variable here

*/


/// Unroll a `rec` type, exposing its recursive body's type structure.
///
/// ### Example 1:  
///
/// `unroll_type(rec a. 1 + a)`  
///  = `1 + (rec a. 1 + (rec a. 1 + a))`
///
/// ### Example 2:
///
/// `unroll_type(rec a. (+ 1 + a + (x a x a)))`  
///  = `(+ 1`  
///      `+ (rec a. 1 + a + (x a x a))`  
///      `+ (x (rec a. 1 + a + (x a x a)) x (rec a. 1 + a + (x a x a)))`  
///     `)`  
///
///
pub fn unroll_type(typ:&Type) -> Type {
    match typ {
        // case: rec x.A =>
        &Type::Rec(ref x, ref a) => {
            // [(rec x.A)/x]A
            subst::subst_type_type(typ.clone(), x, (**a).clone())
        }
        // error
        _ => {
            //println!("error: not a recursive type; did not unroll it: {:?}", typ);
            typ.clone()
        }
    }
}
