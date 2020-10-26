#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn swap(mut self, first: u8, second: u8) -> Color {
        // match first {
        //     v if v == self.r => {
        //         match second {
        //             v if v == self.b => {
        //                 self.r = second;
        //                 self.b = first;
        //             },
        //             v if v == self.g => {
        //                 self.r = second;
        //                 self.g = first;
        //             },
        //             v if v == self.a => {
        //                 self.r = second;
        //                 self.a = first;
        //             },
        //             _ => {},
        //         }
        //     },
        //     _ => {}
        // }
        let x1 = (first == self.r, first == self.g, first == self.b, first == self.a);
        let x2 = (second == self.r, second == self.g, second == self.b, second == self.a);
        
        match (x1, x2) {
            ((true, _, _, _), (_, true, _, _)) => {
                self.r = second;
                self.b = first;
                }
            ((true, _, _, _), (_, _, true, _)) => {
                self.r = second;
                self.g = first;
                }
            ((true, _, _, _), (_, _, _, true)) => {
                self.r = second;
                self.a = first;
            }
            _ => {}
        }
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
fn main() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };

    let color = c.swap(c.b, c.a);

    println!("{:?}", color);
}