#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const N: usize = 400;
const M: usize = 1995;

const SIDE: usize = 800;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
    fn from_usize_pair(p: (usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    fn in_field(&self) -> bool {
        (0 <= self.x && self.x <= SIDE as isize) && (0 <= self.y && self.y <= SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    // fn specific_distance(&self, that: &Self) -> isize {
    //     (self.x - that.x).abs() + (self.y - that.y).abs()
    // }

    // 問題用のユークリッド距離(di)
    fn specific_distance(&self, that: &Self) -> usize {
        let xx = (self.x as f64 - that.x as f64).powi(2);
        let yy = (self.y as f64 - that.y as f64).powi(2);

        (xx + yy).sqrt().round() as usize
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
            .collect()
    }

    fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }
}

struct Input {
    xy: Vec<Coord>,
    uv: Vec<(usize, usize)>,
    l: Vec<usize>,
}
impl Input {
    fn new(xy: Vec<Coord>, uv: Vec<(usize, usize)>) -> Self {
        Self {
            xy,
            uv,
            l: Vec::with_capacity(M),
        }
    }
}

fn main() {
    let system_time = SystemTime::now();
    let mut rng = thread_rng();

    // input
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let mut xy = Vec::with_capacity(N);
    let mut uv = Vec::with_capacity(M);
    for _ in 0..N {
        let x: usize = sc.read();
        let y: usize = sc.read();
        xy.push(Coord::from_usize_pair((x, y)));
    }
    for _ in 0..M {
        let u: usize = sc.read();
        let v: usize = sc.read();
        uv.push((u, v));
    }

    let mut input = Input::new(xy, uv);

    // main
    let mut uf = UnionFind::new();

    // main loop
    for i in 0..M {
        let l: usize = sc.read();
        input.l.push(l);

        let (u, v) = input.uv[i];

        // 2*di 以下が対象
        if !uf.is_connect(u, v) && l <= input.xy[u].specific_distance(&input.xy[v]) * 2 {
            println!("{}", 1);
            uf.connect(u, v);
        } else {
            println!("{}", 0);
        }
    }

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

struct UnionFind {
    uni: Vec<isize>, // 根であれば *そのグループの要素数(負)* が、子であれば親の番号が入る。
}

#[allow(dead_code)]
impl UnionFind {
    fn new() -> Self {
        UnionFind { uni: vec![-1; N] }
    }

    // 頂点 v の所属するグループを調べる
    fn root(&mut self, v: usize) -> usize {
        if self.uni[v] < 0 {
            v
        } else {
            self.uni[v] = self.root(self.uni[v] as usize) as isize;
            self.uni[v] as usize
        }
    }

    // 頂点 a と頂点 b を繋ぐ。元々同じグループのとき　false を返す
    fn connect(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.root(a) as usize;
        let mut root_b = self.root(b) as usize;

        if root_a == root_b {
            return false;
        }

        // a 側が大きいグループになるようにスワップ
        if self.uni[root_a] > self.uni[root_b] {
            root_a ^= root_b;
            root_b ^= root_a;
            root_a ^= root_b;
        }

        // root_a と root_b を結合し、root_b の親を root_a とする
        self.uni[root_a] += self.uni[root_b];
        self.uni[root_b] = root_a as isize;

        return true;
    }

    // 頂点 a, b が同じグループであるかを調べる
    fn is_connect(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    // 頂点 v を含むグループの頂点数を調べる
    fn size(&mut self, v: usize) -> usize {
        let root = self.root(v);
        self.uni[root].abs() as usize
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
