#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const N: usize = 400;
const M: usize = 1995;

const SIDE: usize = 800;

const WORLD_NUM: usize = 47;

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
    let agree_line: usize = std::env::var("AGREE_LINE")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let system_time = SystemTime::now();
    let mut rng = thread_rng();

    // input
    let (mut input, ls) = read_file("./tools/in/0000.txt".to_string());

    // main
    let mut uf = kruskal::UnionFind::new(N);

    // クラスカル用にデータ構造整える
    let mut worlds = vec![];
    for _ in 0..WORLD_NUM {
        let mut edges = Vec::with_capacity(M);
        for mi in 0..M {
            let (u, v) = input.uv[mi];

            let di = input.xy[u].specific_distance(&input.xy[v]);
            let estimated_cost = rng.gen_range(1.13, 2.87) * di as f64;

            edges.push((MinNonNan(estimated_cost), (u, v), mi));
        }

        edges.sort();
        worlds.push(edges);
    }

    // main loop
    let mut score = 0;
    for mi in 0..M {
        // すでに連結な辺を間引く
        if mi % 100 == 0 {
            for edges in &mut worlds {
                edges.retain(|(_, (u, v), _)| !uf.is_connect(*u, *v));
            }
        }

        let (u, v) = input.uv[mi];

        // エッジmiのコスト
        let l: usize = ls[mi];
        input.l.push(l);

        let lc = MinNonNan(l as f64);

        // 各世界線での多数決
        let mut agree_cnt = 0;
        if !uf.is_connect(u, v) {
            for edges in &mut worlds {
                // edges の上書き作業
                edges.retain(|e| e.2 != mi);
                let mut flag = false;
                for i in 0..edges.len() {
                    if edges[i].0 >= lc {
                        edges.insert(i, (lc, (u, v), mi));
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    edges.push((lc, (u, v), mi));
                }

                // MST
                let res = kruskal::calc(&edges, (lc, (u, v)), uf.clone());
                if res.contains(&(u, v)) {
                    agree_cnt += 1;
                }
            }
        }

        if agree_cnt >= agree_line {
            uf.connect(u, v);
            score += l;
        }

        for edges in &mut worlds {
            edges.retain(|e| e.2 != mi);
        }
    }

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
    println!("cost: {}", score);
}

#[derive(PartialEq, Clone, Copy)]
pub struct MinNonNan(f64);

impl Eq for MinNonNan {}
impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// 重み付き連結無向グラフが対象(0-based index)
mod kruskal {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    const N: usize = super::N;

    // MSTを成すエッジ列を返す
    pub fn calc(
        edges: &Vec<(super::MinNonNan, (usize, usize), usize)>, // (cost, (s, t), mi): s-t を繋ぐエッジとそのcost
        now_target_edge: (super::MinNonNan, (usize, usize)),    // (cost, (s, t))
        mut uf: UnionFind,
    ) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::with_capacity(super::N - 1);

        let mut pq = BinaryHeap::new();

        for e in edges {
            pq.push(Reverse(e.clone()));
        }

        for i in 0..edges.len() {
            let (_, (s, t), _) = edges[i];
            if !uf.is_connect(s, t) {
                uf.connect(s, t);
                res.push((s, t));

                // 連結になったら打ち切る
                if uf.size(s) == N {
                    break;
                }
            }
            if (s, t) == now_target_edge.1 {
                break;
            }
        }

        res
    }

    #[derive(Clone)]
    pub struct UnionFind {
        uni: Vec<isize>, // 根であれば *そのグループの要素数(負)* が、子であれば親の番号が入る。
    }
    #[allow(dead_code)]
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            UnionFind { uni: vec![-1; n] }
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
        pub fn connect(&mut self, a: usize, b: usize) -> bool {
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
        pub fn is_connect(&mut self, a: usize, b: usize) -> bool {
            self.root(a) == self.root(b)
        }
        // 頂点 v を含むグループの頂点数を調べる
        fn size(&mut self, v: usize) -> usize {
            let root = self.root(v);
            self.uni[root].abs() as usize
        }
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
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

#[allow(dead_code, unused)]
fn read_file(file_path: String) -> (Input, Vec<usize>) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    // ここにファイル内容を書き込む
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    // contents をパースして、入力を作れ
    let v = contents.split("\n").collect::<Vec<_>>();
    let mut xy = vec![];
    for i in 0..N {
        let p = v[i]
            .split(" ")
            .collect::<Vec<_>>()
            .iter()
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let pos = Coord::from_usize_pair((p[0], p[1]));

        xy.push(pos);
    }

    let mut uv = vec![];
    for i in N..N + M {
        let p = v[i].split(" ").collect::<Vec<_>>();
        let pp = (
            p[0].parse::<usize>().unwrap(),
            p[1].parse::<usize>().unwrap(),
        );

        uv.push(pp);
    }

    let mut ls = vec![];
    for i in N + M..(N + M) + M {
        let l = v[i].parse::<usize>().unwrap();
        ls.push(l);
    }

    (Input::new(xy, uv), ls)
}
