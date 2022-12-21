use std::fmt;
use std::fmt::{Display, Formatter, Error};///different types inthe namespace

enum VertDir{ ///VertDir is defined as an enum
    Up, 
    Down
}

enum HorizDir{ ///HorizDir is defined as an enum
    Left, 
    Right
}

///Its important know the balls x and y positions and its vertical and horizontal directions in order to track the ball
struct Ball{
    x : i32, 
    y : i32,
    vert_dir : VertDir, 
    horiz_dir : HorizDir
}


struct Frame{     ///Defines Frame as a struct
    width : i32, 
    height : i32, 
}


struct Game {     ///Defines Game as a struct 
    frame : Frame, 
    ball : Ball
}

impl Game { ///Defining a new method using the Game type
    fn new()->Game{
     
     Game{ ///Establishes frame measurements for game
            frame : Frame{
            width : 63, 
            height : 31 
            },

            ball: Ball{ ///Ball positioning
                x : 44,
                y : 21, 
                vert_dir : VertDir::Up,
                horiz_dir : HorizDir::Right 
            }
     }
    }

    fn step(&mut self){
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }

}


impl Ball{ ///Implements ball bouncing logic using Ball type
    fn bounce(&mut self, frame: &Frame){
        if self.x <= 0 { 
            
            self.horiz_dir = HorizDir::Right;
        }
        else if frame.width<=self.x{
            self.horiz_dir = HorizDir::Left;
        }
        else if self.y<=0{ 
            
            self.vert_dir = VertDir::Down;
        }
        else if frame.height<=self.y{
            self.vert_dir = VertDir::Up;
        }

        else{
               
        }

    }
///additional method within impl Ball
    fn mv(&mut self){ 
        match self.horiz_dir{
            HorizDir::Left => self.x-=1,
            HorizDir::Right => self.x+=1
        }
        match self.vert_dir{
            VertDir::Up => self.y-=1,
            VertDir::Down => self.y+=1
        }
    
    }
}


impl Display for Game{ 
    fn fmt(&self, fmt:&mut Formatter)->Result<(), Error>{
        write!(fmt, "x");
        for _ in 0..64{ write!(fmt, "-"); } 
        for y in 0..32{
            for x in 0..64{
                if self.ball.x == x as i32 && self.ball.y == y as i32{
                    write!(fmt, "O"); 
                    
                }
                if x == 0 {write!(fmt, "|");} else if x!=0 && y!=31 {write!(fmt, " ");} else {write!(fmt,"-");}
            }
            write!(fmt, "|\n");
        }

        write!(fmt, "\n")
       // write!(fmt, "{} {}", self.ball.x, self.ball.y)
    }
}

///Infinite loop created within main
fn main() {
     let mut new_game  = Game::new();
     let sleep_time = std::time::Duration::from_millis(33);
     loop {
            println!("{}", new_game); 
            new_game.step();
            std::thread::sleep(sleep_time);  
            println!("{} {} ", new_game.ball.x, new_game.ball.y);
     }


}
