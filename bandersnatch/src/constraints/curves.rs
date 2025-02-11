use crate::{constraints::FqVar, *};
use ark_r1cs_std::groups::curves::{
    short_weierstrass::ProjectiveVar, twisted_edwards::AffineVar,
};

/// A variable that is the R1CS equivalent of `crate::EdwardsAffine`.
pub type EdwardsVar = AffineVar<BandersnatchParameters, FqVar>;

/// A variable that is the R1CS equivalent of `crate::EdwardsProjective`
pub type SWVar = ProjectiveVar<BandersnatchParameters, FqVar>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::te_test::<_, EdwardsVar>().unwrap();
    ark_curve_constraint_tests::curves::sw_test::<BandersnatchParameters, SWVar>()
        .unwrap();
    ark_curve_constraint_tests::curves::group_test::<
        EdwardsProjective,
        Fq,
        EdwardsVar,
    >()
    .unwrap();
}
