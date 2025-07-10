use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        // RT10xx and RT11xx chips.
        // rt1xxx: {
        //     any(
        //         feature = "mimxrt1011",
        //         feature = "mimxrt1062"
        //     )
        // },
    }
}
