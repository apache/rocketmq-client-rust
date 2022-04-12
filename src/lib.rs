pub mod apache {
    pub mod rocketmq {
        pub mod v1 {
            tonic::include_proto!("apache.rocketmq.v1");
        }
    }
}

pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

pub mod org_apache_rocketmq {

    #[derive(Debug)]
    struct RpcClient {}
}

#[cfg(test)]
pub mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4)
    }
}
