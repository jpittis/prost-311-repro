pub mod repro {
    include!(concat!(env!("OUT_DIR"), "/repro.rs"));
    pub mod repro {
        include!(concat!(env!("OUT_DIR"), "/repro.repro.rs"));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
