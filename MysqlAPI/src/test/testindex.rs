use super::{
    exceltest::importExcle, jsontest::testjson, redistest::test_redis, testdemo::test_rust,
};

pub fn Test() {
    //test_rust();
    //test_redis();
    testjson();
    importExcle();
}
