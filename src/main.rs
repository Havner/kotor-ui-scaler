use std::path::Path;

fn two_files(file1: &Path, file2: &Path, fileo: &Path, mult: f64)
{
    let bytes1 = std::fs::read(file1).expect(&format!("cannot read {}", file1.display()));
    let bytes2 = std::fs::read(file2).expect(&format!("cannot read {}", file2.display()));

    if bytes1.len() != bytes2.len() {
        panic!("{} & {} differ in length", file1.display(), file2.display());
    }

    let mut byteso: Vec<u8> = Vec::with_capacity(bytes1.len());

    let mut counter = 0;
    let together = bytes1.chunks_exact(2).zip(bytes2.chunks_exact(2)).enumerate();
    for elem in together {
        let pos = elem.0;
        let elem = elem.1;
        let short1 = u16::from_le_bytes([elem.0[0], elem.0[1]]);
        let short2 = u16::from_le_bytes([elem.1[0], elem.1[1]]);

        if short1 != short2 {
            let shorto = (short2 as f64 * mult) as u16;
            // match counter {
            //     0 => print!("{:X?}: {}, ", pos, shorto),
            //     3 => println!("{}", shorto),
            //     _ => print!("{}, ", shorto),
            // }
            counter = (counter + 1) % 4;

            let elemo = shorto.to_le_bytes();
            byteso.push(elemo[0]);
            byteso.push(elemo[1]);
        } else {
            byteso.push(elem.0[0]);
            byteso.push(elem.0[1]);
        }
    }
    // println!("");

    // quick and ugly odd length handling
    if byteso.len() != bytes2.len() {
        byteso.push(bytes2[bytes2.len()-1]);
    }
    if byteso.len() != bytes2.len() {
        panic!("{} & {} final length still wrong", file2.display(), fileo.display());
    }

    std::fs::write(fileo, byteso).expect(&format!("writing {} failed", fileo.display()));
}

fn handle_dirs(dir1: &str, dir2: &str, diro: &str, mult: f64)
{
    let names = std::fs::read_dir(dir1).expect(&format!("cannot read {}", dir1));
    for name in names {
        let name = name.unwrap().path();
        let filename = Path::new(name.file_name().unwrap());
        println!("{}", filename.display());
        let file1 = Path::new(dir1).join(&filename);
        let file2 = Path::new(dir2).join(&filename);
        let fileo = Path::new(diro).join(&filename);
        two_files(&file1, &file2, &fileo, mult);
    }
}

fn main()
{
    let dir1 = "gui.1920x1200";
    let dir2 = "gui.2880x1800";
    let diro = "gui.3072x1920";
    //let res1 = 1920;
    let res2 = 2880;
    let reso = 3072;
    let mult: f64 = reso as f64 / res2 as f64;

    handle_dirs(dir1, dir2, diro, mult);

    println!("{}", mult);
}
