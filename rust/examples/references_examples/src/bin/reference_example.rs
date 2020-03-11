use references_examples::placeholder;

fn main() {
    let mut v: Vec<&'static str> = Vec::new();
    v.push("v");

    {
        // only can exist unique mutable reference or multi immutable reference
        let r_1 = &v;
        let r_2 = &v;

        /* ---------------- Incorrect Code ---------------- */

        // let r_mut = &mut v; // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

        /* ---------------- Incorrect Code ---------------- */

        placeholder(r_1); // immutable borrow later used here
        placeholder(r_2); // immutable borrow later used here

        // r_1 and r_2 destroy here
        let r_mut = &mut v; // can borrow as mutable when immutable reference destroied
        r_mut.push("r_mut after immutable ref gone");
    }

    // move ownership of reference to `r_mut` from tmp
    let tmp_mut = &mut v;
    let mut r_mut = tmp_mut;

    /* ---------------- Incorrect Code ---------------- */

    // placeholder(tmp_mut); // error[E0382]: use of moved value: `tmp_mut`

    /* ---------------- Incorrect Code ---------------- */

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

    // move ownership as mutable
    let mut v_1 = v;

    /* ---------------- Incorrect Code ---------------- */

    // placeholder(r_mut); // error[E0505]: cannot move out of `v` because it is borrowed here
    // r_mut = &v_1; // error[E0308]: mismatched types

    /* ---------------- Incorrect Code ---------------- */

    r_mut = &mut v_1; // reference to another value because r_mut is a mutable reference
    r_mut.push("r_mut to v_1");

    // move ownership as immutable
    let v_imut = v_1;

    /* ---------------- Incorrect Code ---------------- */

    // r_mut = &mut v_imut; // error[E0596]: cannot borrow `v_imut` as mutable, as it is not declared as mutable

    /* ---------------- Incorrect Code ---------------- */

    println!("v={:?}", v_imut);
}
