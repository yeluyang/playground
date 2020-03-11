use std::{
    borrow::{Borrow, BorrowMut},
    rc::Rc,
};

use references_examples::placeholder;

fn main() {
    let mut v: Vec<&'static str> = Vec::new();
    v.push("v");

    // `Rc<T>` will take ownership and share it
    let mut rc = Rc::new(v);

    /* ---------------- Incorrect Code ---------------- */

    // rc.push("rc to v"); // error[E0596]: cannot borrow data in an `Rc` as mutable

    /* ---------------- Incorrect Code ---------------- */

    // only can exist unique mutable reference or multi immutable reference
    {
        let r_1: &Vec<&'static str> = rc.borrow(); // `Rc<T>` only can borrow as immutable reference
        let r_2: &Vec<&'static str> = rc.borrow(); // `Rc<T>` only can borrow as immutable reference

        /* ---------------- Incorrect Code ---------------- */

        // let r_mut = Rc::get_mut(&mut rc).unwrap(); // error[E0502]: cannot borrow `rc` as mutable because it is also borrowed as immutable

        /* ---------------- Incorrect Code ---------------- */

        placeholder(r_1); // immutable borrow later used here
        placeholder(r_2); // immutable borrow later used here

        // r_1 and r_2 destroy here
        let r_mut = Rc::get_mut(&mut rc).unwrap(); // can borrow as mutable when immutable reference destroied
        r_mut.push("r_mut after immutable ref gone");
    }

    let r_mut = Rc::get_mut(&mut rc).unwrap(); // can borrow as mutable only when no other `Rc<T>` reference same data
    r_mut.push("r_mut to v");

    // `r_mut` is re-borrowed by r_mut_1, and become inacessible while r_mut_1 is alived
    {
        let r_mut_re = &mut *r_mut;

        /* ---------------- Incorrect Code ---------------- */

        // placeholder(r_mut); // error[E0502]: cannot borrow `*r_mut` as immutable because it is also borrowed as mutable

        /* ---------------- Incorrect Code ---------------- */

        r_mut_re.push("r_mut_re re-borrow from r_mut");
    }
    r_mut.push("r_mut after re-borrow"); // `r_mut` is accessible again

    {
        placeholder(r_mut);
        // mutable reference `r_mut` destroy here
        let r: &Vec<&'static str> = rc.borrow(); // can clone `Rc<T>` when it only borrow as immutable
        let rc_1 = rc.clone(); // `Rc<T>` can share ownership to anothers
        let rc_2 = rc_1.clone(); // `Rc<T>` can share ownership like chain
        placeholder(r);

        /* ---------------- Incorrect Code ---------------- */

        // placeholder(r_mut); // error[E0502]: cannot clone `rc` because it is also borrowed as mutable and used here

        /* ---------------- Incorrect Code ---------------- */

        placeholder(&rc_1);
        placeholder(&rc_2);
        assert!(Rc::get_mut(&mut rc).is_none()); // cannot borrow as mutable because some other rc alived

        let mut rc_mut = rc.clone();
        let r_mut_clone = Rc::make_mut(&mut rc_mut); // it will clone inner behind `rc_mut` and update inner of `rc_mut` as new cloned, when other `Rc<T>` refer same data, then return its mutable reference
        r_mut_clone.push("r_mut_clone created by Rc::make_mut");
        println!("r_mut_clone={:?}", r_mut_clone);
    }
    // `rc_1` and `rc_2` destroy here
    let r_mut = Rc::get_mut(&mut rc).unwrap(); // can borrow as mutable again
    r_mut.push("r_mut after Rc::clone()");

    println!("rc={:?}", rc);
}
