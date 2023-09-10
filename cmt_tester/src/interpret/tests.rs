
use cement::preclude::*;

use super::structs::ItprtMod;

cmt::new_bundle! { c =>
    struct Delay2 {
        clk: Var<Clk>,
        inp: Var<U8t>,
        outp: MutVar<U8t>,
    }

    fn new() -> Self {
        Self {
            clk: Clk.to(c),
            inp: U8.to(c),
            outp: U8.to(c).into_mut(),
        }
    }

    fn build(self) {
        let (r_r, r_w) = U8.reg(c, self.clk);
        r_w <<= self.inp.reg(c, self.clk);
        self.outp <<= r_r;
    }
}

#[test]
fn print_startup() {

    let mut c = Cmtc::new(true);

    Delay2::new_top(&mut c);

    c.print();

    let itprt_mod = ItprtMod::from_cmt(&c.env, c.top_module());

    println!("{:#?}", itprt_mod)
} 