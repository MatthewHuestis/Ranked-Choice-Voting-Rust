pub mod voting_system{
    use std::io::{Error, ErrorKind};

    pub fn start_system(){
        let mut machine=VotingSystem::new();
        match machine.read_votes(){
            Ok(())=>machine.tally_votes(),
            Err(error)=>panic!("ERROR!:{error}")
        }
        
    }

    struct VotingSystem{
        //vector of an size 6 array that stores all voter data for later user
        votes : Vec<[u16;6]>,
        //2d array to store the primay votes of each voter in each county
        county_votes : [[u64;5];124],
        //variable for total votes
        tally : u64,
        //number of iteration of votes
        iter: u8
    }

    
    impl VotingSystem{
        //constructor for voting system;
        fn new()->VotingSystem{
            VotingSystem{tally : 0, votes: Vec::new(), county_votes : [[0;5];124], iter : 1}
        }

        //function to read the votes from the data file
        fn read_votes(&mut self)->Result<(), Error> {

            //creating file iterator variable
            let file = std::fs::read_to_string("vote.txt")?;
            let mut error:Error=Error::new(ErrorKind::Other,"No issue");
            let mut pass=true;
            //iterates through each line of the file.
            if file==String::new() {
                pass=false;
                error = Error::new(ErrorKind::InvalidInput, "vote.txt not found or does not exist");
            }

            else {
                for line in file.lines(){
                    //in case of bad value
                    if line == String::new() {
                    continue;
                    }
                
                    else {
                    self.tally+=1;
                    //splitn it for the file line
                    let mut split = line.splitn(6, " ");
                    //in[puts data into a vector with parsed data.
                    //messy but gets the job done
                    let data:[u16;6]=[(split.next().unwrap()).parse::<u16>().unwrap(),
                        (split.next().unwrap()).parse::<u16>().unwrap(),
                        (split.next().unwrap()).parse::<u16>().unwrap(),
                        (split.next().unwrap()).parse::<u16>().unwrap(),
                        (split.next().unwrap()).parse::<u16>().unwrap(),
                        (split.next().unwrap()).parse::<u16>().unwrap()];

                    //Takes data and adds to county vote for traking counties primary vote
                    if data[1]==1 {self.county_votes[data[0]as usize][0]+=1;}
                    else if data[2]==1 {self.county_votes[data[0]as usize][1]+=1;}
                    else if data[3]==1 {self.county_votes[data[0]as usize][2]+=1;}
                    else if data[4]==1 {self.county_votes[data[0]as usize][3]+=1;}
                    else if data [5]==1 {self.county_votes[data[0]as usize][4]+=1;}
                     else {
                          //if invalid data is found return an error
                          pass=false;
                          error=Error::new(ErrorKind::InvalidData, "Data corruption after reading file");
                          
                         }
                    self.votes.push(data);
                    }
                }
            }
            //checks for errors
            match pass{
                true=>Ok(()),
                false=>Err(error)
            }

        }

        fn tally_votes(&mut self) {
            //array to carry 
            let mut cand:[u64;5]=[0;5];

            let v_it = (self.votes).iter();
            
            for data in v_it{
                if data[1]==1{cand[0]+=1}
                else if data[2]==1{cand[1]+=1}
                else if data[3]==1{cand[2]+=1}
                else if data[4]==1{cand[3]+=1}
                else if data[5]==1{cand[4]+=1}
                else{println!("Read error")};
            }

            //array for vote percentages
            let mut cand_perc:[f64;5]=[0.;5];
            //pass value
            let mut pass=false;

            for i in 0..5 {
                cand_perc[i]=(cand[i] as f64)/(self.tally as f64);
            }
            //displays result of iteration
            println!("Iteration {},\n\tA:{}% B:{}% C:{}% D:{}% E:{}%",self.iter,cand_perc[0]*100.,
                cand_perc[1]*100., cand_perc[2]*100., cand_perc[3]*100., cand_perc[4]*100. );
            //finds if there is a winner and announces if it has
            if cand_perc[0]>=0.5 {println!("Candiate A Wins!")}
            else if cand_perc[1]>=0.5 {println!("Candiate B Wins!"); pass=true;}
            else if cand_perc[2]>=0.5 {println!("Candiate C Wins!"); pass=true;}
            else if cand_perc[3]>=0.5 {println!("Candiate D Wins!"); pass=true;}
            else if cand_perc[4]>=0.5 {println!("Candiate E Wins!"); pass=true;}
            //default case in which no canidate has more than %50
            else{
                self.iter+=1;
                let mut k=0;
                for i in 0..5{
                    if cand_perc[i]<cand_perc[k]&& cand_perc[i]!=0. {
                        k=i;
                    }
                }
                self.drop_cand(k+1);
            }

            if pass==true {
                
            }
        }

        //function to drop the candiate with lowest percentage of votes
        fn drop_cand(&mut self, drop:usize){
            let it = self.votes.iter_mut();

            for data in it{
                let k=data[drop];
                for i in 1..6{
                    if data[i] > k {data[i] -= 1;}
                    else if data[i]==k {data[i] = 0;}
                }
            }
            self.tally_votes();
        }

    }  


}