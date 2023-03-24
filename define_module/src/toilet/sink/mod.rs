pub fn clean_sink() {
    println!("cleaning sink");
}

pub mod soap {
    pub fn soap_clean() {
        println!("soap cleaning");

        // this is private in parent module
        super::super::super::toilet::private_flush();
    }
}
