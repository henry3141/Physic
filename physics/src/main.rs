use quicksilver::{
    geom::{Circle, Vector},
    graphics::{Color, Graphics},
    Input, Window, Result, Settings, run,
};
use std::{thread, time};
use std::rc::Rc;

fn main() {
    run(
        Settings {
            title: "E = mc^2",
            ..Settings::default()
        },
        app,
    );
}


struct Atom {
    position:[f32;2],
    velocity:[f32;2],
    mass:f32,
}

impl Atom {
    fn new(pos:[i32;2] , mass:i32) -> Atom {
        Atom { position:[pos[0] as f32 , pos[1] as f32] , mass:mass as f32 , velocity:[0 as f32 , 0 as f32]}
    }

    fn update(self , atoms:&Vec<Atom>) -> Atom { 
        let mut ra = Atom {position:self.position , velocity:self.velocity , mass:self.mass};
        for i in &atoms {
            let mut vec = [ra[0] - i.position[0],ra[1] - i.position[1]];
            let lenght = (vec[0] ** 2 + vec[1] ** 2) as f32;
            let lenght = lenght
        }


    }
}

struct World {
    atoms:Vec<Atom>,
}

impl World {
    fn empty() -> World {
        World { atoms:vec![] }
    }

    fn draw(&self,gfx:&mut Graphics) {
        for i in &self.atoms {
            let circle = Circle::new(Vector::new(i.position[0],i.position[1]),i.mass);
            gfx.fill_circle(&circle,Color::WHITE);
        }
    }

    fn add(self,atom:Atom) {
        self.atoms.push(atom);
    }
    
    fn update(&mut self) {
        let mut atoms = vec![];
        for i in &self.atoms { 
            atoms.push(thread::spawn(||{i.update(&self.atoms)}));
        }
        let mut index = 0;
        for i in atoms {
            self.atoms[index] = i.join().unwrap();
            index = index + 1;
        }
    }

}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mut world = World::empty();
    world.add(Atom::new([100,100],20));
    let fps_time = time::Duration::from_millis(1000/30);
    loop {
        let now = time::Instant::now();
        gfx.clear(Color::BLACK);
        world.draw(&mut gfx);
        gfx.present(&window)?;
        let elapsed_time = now.elapsed();
        if elapsed_time < fps_time {
            loop {
                while let Some(_) = input.next_event().await {}
                if elapsed_time > fps_time {
                    break;
                }
            }
        }
    }
}
