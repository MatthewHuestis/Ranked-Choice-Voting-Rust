pub mod voting_System{

    struct voting_system{
        //Original data strucutes from original project moved to the struct
        votes : vec<[u16;6]>,
        county_votes : vec<vec<u16,6>,124>,
        tally : u64,
        iter: u8
    }
    
    impl voting_system{

        fn read_Votes(&mut self)->Result((),Error) {
            //creating file iterator variable
            let file = std::fs::read_to_string("votes.txt")?;

            for line in file.lines(){
                

            }

        }

        fn tally_Votes(&self){

        }
        fn drop_cand(&mut self, drop:u8){

        }

    }  


}