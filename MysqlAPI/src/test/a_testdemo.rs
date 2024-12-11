use super::{
    demotest::test_rust, exceltest::importExcle, jsontest::testjson, redistest::test_redis,
};

pub fn Test() {
    //test_rust();
    //test_redis();
    testjson();
    importExcle();
}
