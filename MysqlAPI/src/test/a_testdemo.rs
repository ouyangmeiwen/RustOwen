use super::{
    demotest::test_rust, exceltest::import_excle, jsontest::testjson, redistest::test_redis,
};

pub fn Test() {
    //test_rust();
    //test_redis();
    testjson();
    //importExcle();
}
