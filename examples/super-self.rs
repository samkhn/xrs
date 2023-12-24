fn function() {
    println!("function (outside current mod)")
}

mod a {
    pub fn function() {
        println!("a::function()")
    }
}

mod b {
    fn function() {
        println!("b::function()")
    }

    mod sub_b {
        pub fn function() {
            println!("b::sub_b::function()")
        }
    }

    pub fn indirect_call() {
        println!("B::indirect_call()");
        // self::function == function
        println!("self::function:");
        self::function();
        println!("function:");
        function();
        // self::sub_b::function == sub_b::function
        println!("self::sub_b::function:");
        self::sub_b::function();
        println!("sub_b::function:");
        sub_b::function();
        // outside current scope
        println!("super::function:");
        super::function();
        {
            // grab cargo scope func
            use crate::a::function as a_function;
            println!("crate::a::function:");
            a_function();
        }
    }
}

fn main() {
    b::indirect_call();
}
