macro_rules! debug {
    ($($e:expr),*) => {{
        use std::io::prelude::*;
        writeln!(::std::io::stderr(), $($e,)*).unwrap();
    }}
}
macro_rules! parse_line {
    ($lines:expr, ($($t:ty),*)) => {{
        let line = $lines.next().unwrap();
        let mut iter = line.split(" ");
        (
            $(iter.next().unwrap().parse::<$t>().unwrap(),)*
        )
    }};
    ($lines:expr, $cons:expr, ($($t:ty),*)) => {{
        let line = $lines.next().unwrap();
        let mut iter = line.split(" ");
        $cons(
            $(iter.next().unwrap().parse::<$t>().unwrap(),)*
        )
    }};
    ($lines:expr, $t:ty) => {{
        $lines.next().unwrap().parse::<$t>().unwrap()
    }};
}

#[cfg(feature = "checker")] mod checker;
#[cfg(feature = "local")] mod local;
#[cfg(feature = "prng")] mod prng;
#[cfg(feature = "tweaker")] mod tweaker;

#[cfg(feature = "local")] fn main() { local::main(); }
#[cfg(feature = "checker")] fn main() { checker::main(); }
#[cfg(feature = "tweaker")] fn main() { tweaker::main(); }

#[cfg(not(feature = "offline"))] fn main() { cg::main(); }

#[allow(dead_code)] mod vec2 {
    use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, Neg};
    use std::f64::consts::PI;
    use std::fmt;
    
    #[inline]
    fn sane_mod(a: f64, b: f64) -> f64 {
        a - (a/b).floor() * b
    }
    
    #[inline]
    fn angle_diff(a: f64, b: f64) -> f64 {
        sane_mod((a - b) + PI, 2. * PI) - PI
    }
    
    #[derive(PartialEq, Copy, Clone)]
    pub struct Vec2(pub f64, pub f64);
    
    impl fmt::Debug for Vec2 {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(fmt, "<{:.2}, {:.2}>", self.0, self.1)
        }
    }
    
    impl Vec2 {
        #[inline]
        pub fn zero() -> Vec2 {
            Vec2::same(0.)
        }
        
        #[inline]
        pub fn same(x: f64) -> Vec2 {
            Vec2(x, x)
        }
        
        #[inline]
        pub fn len(self) -> f64 {
            (self * self).sqrt()
        }
    
        #[inline]
        pub fn len_sq(self) -> f64 {
            self * self
        }
    
        #[inline]
        pub fn distance_sq(self, other: Vec2) -> f64 {
            (other - self).len_sq()
        }
        
        #[inline]
        pub fn norm(self) -> Vec2 {
            let len = self.len();
            self * (1.0 / len)
        }
        
        #[inline]
        pub fn from_angle(angle: f64, len: f64) -> Vec2 {
            Vec2(angle.cos() * len, angle.sin() * len)
        }
        
        #[inline]
        pub fn angle(self) -> f64 {
            self.1.atan2(self.0)
        }
        
        #[inline]
        pub fn distance_to(self, other: Vec2) -> f64 {
            (other - self).len()
        }
        
        #[inline]
        pub fn round(self) -> Vec2 {
            Vec2(self.0.round(), self.1.round())
        }
    
        #[inline]
        pub fn floor(self) -> Vec2 {
            Vec2(self.0.floor(), self.1.floor())
        }
        
        #[inline]
        pub fn perp(self) -> Vec2 {
            Vec2(self.1, -self.0)
        }
    
        #[inline]
        pub fn cross(self, other: Vec2) -> f64 {
            self.0 * other.1 - self.1 * other.0
        }
    
        #[inline]
        pub fn signed_angle_to(self, other: Vec2) -> f64 {
            angle_diff(other.angle(), self.angle())
        }
    
        #[inline]
        pub fn neg(self) -> Vec2 {
            Vec2(-self.0, -self.1)
        }
        
        #[inline]
        pub fn rotate(self, angle: f64) -> Vec2 {
            let x = self.0;
            let y = self.1;
            let c = angle.cos();
            let s = angle.sin();
            Vec2(x * c - y * s, x * s + y * c)
        }
    
        #[inline]
        pub fn is_nan(self) -> bool {
            self.0.is_nan() || self.1.is_nan()
        }
    }

    impl Neg for Vec2 {
        type Output = Vec2;

        #[inline]
        fn neg(self) -> Vec2 {
            Vec2(-self.0, -self.1)
        }
    }
    
    impl Mul for Vec2 {
        type Output = f64;
        
        #[inline]
        fn mul(self, other: Vec2) -> f64 {
            self.0 * other.0 + self.1 * other.1
        }
    }
    
    impl Mul<f64> for Vec2 {
        type Output = Vec2;
        
        #[inline]
        fn mul(self, other: f64) -> Vec2 {
            Vec2(self.0 * other, self.1 * other)
        }
    }
    
    impl Add for Vec2 {
        type Output = Vec2;
        
        #[inline]
        fn add(self, other: Vec2) -> Vec2 {
            Vec2(self.0 + other.0, self.1 + other.1)
        }
    }
    
    impl Sub for Vec2 {
        type Output = Vec2;
        
        #[inline]
        fn sub(self, other: Vec2) -> Vec2 {
            Vec2(self.0 - other.0, self.1 - other.1)
        }
    }
    
    impl AddAssign for Vec2 {
        #[inline]
        fn add_assign(&mut self, other: Vec2) {
            *self = *self + other;
        }
    }
    
    impl SubAssign for Vec2 {
        #[inline]
        fn sub_assign(&mut self, other: Vec2) {
            *self = *self - other;
        }
    }
}
mod config {
    #[cfg(feature = "draw")]
    pub const DRAWING_ENABLED: bool = true;
    #[cfg(not(feature = "draw"))]
    pub const DRAWING_ENABLED: bool = false;

    #[cfg(feature = "checker")]
    pub const VERBOSE: bool = true;
    #[cfg(not(feature = "checker"))]
    pub const VERBOSE: bool = false;

    #[cfg(feature = "tweaker")]
    pub const QUIET: bool = true;
    #[cfg(not(feature = "tweaker"))]
    pub const QUIET: bool = false;
}
mod consts {
    use vec2::*;
    
    // Game constants
    pub const WIZARD_COUNT: usize = 2;
    pub const TOTAL_WIZARD_COUNT: usize = WIZARD_COUNT * 2;
    pub const WIZARD_RADIUS: f64 = 400.;
    pub const SNAFFLE_RADIUS: f64 = 150.;
    pub const MAX_SNAFFLE_COUNT: usize = 7;
    pub const GOAL_Y: f64 = 3750.;
    pub const GOAL_POSITIONS: [Vec2; 2] = [Vec2(0., GOAL_Y), Vec2(16000., GOAL_Y)];
    pub const POLE_RADIUS: f64 = 300.;
    pub const GOAL_RADIUS: f64 = 2000.;
    pub const BLUDGER_COUNT: usize = 2;
    pub const BLUDGER_RADIUS: f64 = 200.;
    pub const WIZARD_MASS: f64 = 1.;
    pub const SNAFFLE_MASS: f64 = 0.5;
    pub const BLUDGER_MASS: f64 = 8.;
    pub const WIZARD_FRICTION: f64 = 0.75;
    pub const BLUDGER_FRICTION: f64 = 0.9;
    pub const SNAFFLE_FRICTION: f64 = 0.75;
    pub const FLIPENDO_COST: i64 = 20;
    pub const ACCIO_COST: i64 = 20;
    pub const OBLIVIATE_COST: i64 = 5;
    pub const PETRIFICUS_COST: i64 = 10;
    pub const FLIPENDO_DURATION: usize = 3;
    pub const ACCIO_DURATION: usize = 6;
    pub const OBLIVIATE_DURATION: usize = 3;
    pub const PETRIFICUS_DURATION: usize = 1;
    pub const MAX_ENTITY_COUNT: usize = TOTAL_WIZARD_COUNT + MAX_SNAFFLE_COUNT + BLUDGER_COUNT;
    pub const BORDER_TOP_Y: f64 = 0.;
    pub const BORDER_BOTTOM_Y: f64 = 7500.;
    pub const BORDER_LEFT_X: f64 = 0.;
    pub const BORDER_RIGHT_X: f64 = 16000.;
    pub const WIZARD_POS: [Vec2; 4] = [Vec2(1000., 5250.), Vec2(1000., 2250.), Vec2(15000., 2250.), Vec2(15000., 5250.)];
    pub const BLUDGER_POS: [Vec2; 2] = [Vec2(7450., 3750.), Vec2(8550., 3750.)];
    pub const POLE_POS: [Vec2; 4] = [ Vec2(BORDER_LEFT_X , GOAL_Y - GOAL_RADIUS)
                                    , Vec2(BORDER_LEFT_X , GOAL_Y + GOAL_RADIUS)
                                    , Vec2(BORDER_RIGHT_X, GOAL_Y - GOAL_RADIUS)
                                    , Vec2(BORDER_RIGHT_X, GOAL_Y + GOAL_RADIUS) ];
}
mod math {
    use vec2::*;

    pub fn circle_collision_time(dpos: Vec2, dvel: Vec2, radius_sum: f64) -> Option<f64> {
        let b = 2. * (dvel * dpos);
        if b >= 0. {
            return None;
        }
        let c = dpos * dpos - radius_sum.powi(2);
        if c <= 0. {
            return Some(0.);
        }
        let a = dvel * dvel;
        let disc = b.powi(2) - 4. * a * c;
        if disc < 0. {
            None
        }
        else {
            let aa = 2. * a;
            let dist = disc.sqrt() / aa;
            let offset = -b / aa;
            let (t1, t2) = (offset - dist, offset + dist);
            if t1 >= 0. && t2 >= 0. {
                if   t1 > t2 { Some(t2) }
                else         { Some(t1) }
            }
            else if t1 >= 0. { Some(t1) }
            else if t2 >= 0. { Some(t2) }
            else             { None     }
        }
    }
}
#[cfg(not(feature = "offline"))] mod cg {
    use std::io::prelude::*;
    use std::io;
    
    use std::borrow::Borrow;
    
    use std::time::Instant;
    
    use vec2::*;
    use game::*;
    use ai::*;
    
    pub fn main() {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines().map(Result::unwrap);
        
        let my_id = parse_line!(lines, PlayerId);
        
        let mut ai = AIConfig::new();
        
        let mut turn = TurnState::new(my_id);
        
        let mut sad_wizards_without_snaffles: Vec<(Vec2, EntityId)> = Vec::with_capacity(4);
        
        loop {
            let entity_count = parse_line!(lines, usize);
            
            turn.prepare(entity_count);
            
            sad_wizards_without_snaffles.clear();
            
            for _ in 0..entity_count {
                let (eid, etype, x, y, vx, vy, state) = parse_line!(lines, (EntityId, String, i64, i64, i64, i64, i64));
                let pos = Vec2(x as f64, y as f64);
                let vel = Vec2(vx as f64, vy as f64);
                match etype.borrow() {
                    "WIZARD" => {
                        let wiz = &mut turn.entities[eid];
                        wiz.pos = pos;
                        wiz.vel = vel;
                        wiz.dead = false;
                        wiz.linked = None;
                        if state == 1 {
                            sad_wizards_without_snaffles.push((wiz.pos, wiz.id));
                        }
                    },
                    "OPPONENT_WIZARD" => {
                        let wiz = &mut turn.entities[eid];
                        wiz.pos = pos;
                        wiz.vel = vel;
                        wiz.dead = false;
                        wiz.linked = None;
                        if state == 1 {
                            sad_wizards_without_snaffles.push((wiz.pos, wiz.id));
                        }
                    },
                    "SNAFFLE" => {
                        let snaffle = &mut turn.entities[eid];
                        snaffle.pos = pos;
                        snaffle.vel = vel;
                        snaffle.dead = false;
                    },
                    "BLUDGER" => {
                        let bludger = &mut turn.entities[eid];
                        bludger.pos = pos;
                        bludger.vel = vel;
                        bludger.dead = false;
                    },
                    _ => unreachable!()
                }
            }
            
            // Let's make these sad wizards happy again.
            // Not having a snaffle is a horrible thing for a wizard.
            // Especially when they're standing right on one.
            // We must save the wizards!
            //
            // #GiveWizardsTheirSnaffles2k16
            for sid in turn.snaffle_ids() {
                for &(pos, wid) in sad_wizards_without_snaffles.iter() {
                    if turn.entities[sid].pos == pos {
                        turn.entities[sid].linked = Some(wid);
                        turn.entities[wid].linked = Some(sid);
                    }
                }
            }

            let start = Instant::now();
            
            let actions = ai.decide(&turn);
            
            // Wish there was a better way to predict future bludger targets…
            let mut pred_turn = turn.clone();
            pred_turn.invert();
            let enemy_actions = simple_ai(&pred_turn);
            pred_turn.invert();
            let stats = pred_turn.simulate(&actions, &enemy_actions);
            for (i, j) in stats.collisions {
                if turn.entities[i].is_a(EntityType::Wizard) && turn.entities[j].is_a(EntityType::Bludger) {
                    turn.entities[j].linked = Some(i);
                }
            }

            for (_, act) in actions.into_iter().enumerate() {
                if let Action::Spell(kind, _) = act {
                    turn.my_mp -= kind.cost();
                }
                println!("{}", act.to_string());
            }
            
            turn.my_mp += 1;
            if turn.my_mp > 100 { turn.my_mp = 100; }
            turn.enemy_mp += 1;
            if turn.enemy_mp > 100 { turn.enemy_mp = 100; }
            
            if turn.my_mp < 0 || turn.enemy_mp < 0 {
                panic!("MP calculation went seriously wrong!");
            }
            
            let time = start.elapsed().subsec_nanos() / 1_000;
            
            debug!("Turn took {}us.", time);
            
            turn.round += 1;
        }
    }
}
#[allow(dead_code)] mod draw {
    use vec2::Vec2;

    use std::io;
    use std::io::prelude::*;
    use config;
    
    pub const SCALING: f64 = 0.05;
    pub const ENABLED: bool = config::DRAWING_ENABLED;
    pub const FIELD_OFFSET: Vec2 = Vec2(99.5, 299.5);

    // shiny-pancake --size 811x405

    macro_rules! output {
        () => { io::stdout() }
    }

    pub fn set_fill_color(r: u8, g: u8, b: u8, a: u8) {
        if ENABLED {
            writeln!(output!(), "#FILL_COLOR {:.0} {:.0} {:.0} {:.0}", r, g, b, a).unwrap();
        }
    }

    pub fn set_stroke_color(r: u8, g: u8, b: u8, a: u8) {
        if ENABLED {
            writeln!(output!(), "#STROKE_COLOR {:.0} {:.0} {:.0} {:.0}", r, g, b, a).unwrap();
        }
    }

    pub fn set_stroke_width(width: f64) {
        if ENABLED {
            writeln!(output!(), "#STROKE_WIDTH {}", width).unwrap();
        }
    }

    pub fn no_fill() {
        if ENABLED {
            writeln!(output!(), "#NOFILL").unwrap();
        }
    }

    pub fn no_stroke() {
        if ENABLED {
            writeln!(output!(), "#NOSTROKE").unwrap();
        }
    }
    
    pub fn circle(mut pos: Vec2, radius: f64) {
        if ENABLED {
            pos += FIELD_OFFSET;
            writeln!(output!(), "#CIRCLE {:.0} {:.0} {:.0}", pos.0 * SCALING, pos.1 * SCALING, radius * SCALING).unwrap();
        }
    }
    
    pub fn rect(mut pos: Vec2, size: Vec2) {
        if ENABLED {
            pos += FIELD_OFFSET;
            writeln!(output!(), "#RECT {:.0} {:.0} {:.0} {:.0}", pos.0 * SCALING, pos.1 * SCALING, size.0 * SCALING, size.1 * SCALING).unwrap();
        }
    }
    
    pub fn ellipse(mut pos: Vec2, size: Vec2) {
        if ENABLED {
            pos += FIELD_OFFSET;
            writeln!(output!(), "#ELLIPSE {:.0} {:.0} {:.0} {:.0}", pos.0 * SCALING, pos.1 * SCALING, size.0 * SCALING, size.1 * SCALING).unwrap();
        }
    }

    pub fn line(mut from: Vec2, mut to: Vec2) {
        if ENABLED {
            from += FIELD_OFFSET;
            to += FIELD_OFFSET;
            writeln!(output!(), "#LINE {:.0} {:.0} {:.0} {:.0}", from.0 * SCALING, from.1 * SCALING, to.0 * SCALING, to.1 * SCALING).unwrap();
        }
    }
    
    pub fn start_frame() {
        if ENABLED {
            writeln!(output!(), "#FRAME_START").unwrap();
        }
    }
}
#[allow(dead_code)] mod game {
    use std::str::FromStr;
    use std::string::ToString;
    use std::fmt;
    use std::mem;
    use std::f64;
    
    use std::ops::{Range, Deref, DerefMut};
    
    use draw;
    use consts::*;
    use config::*;
    use vec2::*;
    use math::*;
    
    pub type PlayerId = usize;
    pub type EntityId = usize;
    pub type PoleId = usize;
    pub type SubId = usize;
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Faction {
        Me,
        Enemy,
        Neutral
    }

    impl Faction {
        pub fn invert(self) -> Faction {
            match self {
                Faction::Me    => Faction::Enemy,
                Faction::Enemy => Faction::Me,
                x              => x
            }
        }
    }
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum EntityType {
        Wizard,
        Snaffle,
        Bludger
    }
    
    impl EntityType {
        #[inline]
        pub fn mass(self) -> f64 {
            match self {
                EntityType::Wizard => WIZARD_MASS,
                EntityType::Snaffle => SNAFFLE_MASS,
                EntityType::Bludger => BLUDGER_MASS
            }
        }
        
        #[inline]
        pub fn friction(self) -> f64 {
            match self {
                EntityType::Wizard => WIZARD_FRICTION,
                EntityType::Snaffle => SNAFFLE_FRICTION,
                EntityType::Bludger => BLUDGER_FRICTION
            }
        }
        
        #[inline]
        pub fn radius(self) -> f64 {
            match self {
                EntityType::Wizard => WIZARD_RADIUS,
                EntityType::Snaffle => SNAFFLE_RADIUS,
                EntityType::Bludger => BLUDGER_RADIUS
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Spell {
        caster_id: EntityId,
        target_id: EntityId,
        kind: SpellType,
        casted: usize,
        duration: usize
    }

    impl Spell {
        pub fn new(kind: SpellType, caster_id: EntityId, target_id: EntityId, round: usize) -> Spell {
            Spell {
                kind: kind,
                caster_id: caster_id,
                target_id: target_id,
                casted: round,
                duration: kind.duration()
            }
        }
    }
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SpellType {
        Flipendo,
        Accio,
        Obliviate,
        Petrificus
    }

    impl SpellType {
        pub fn duration(self) -> usize {
            match self {
                SpellType::Flipendo   => FLIPENDO_DURATION,
                SpellType::Accio      => ACCIO_DURATION,
                SpellType::Obliviate  => OBLIVIATE_DURATION,
                SpellType::Petrificus => PETRIFICUS_DURATION
            }
        }

        pub fn cost(self) -> i64 {
            match self {
                SpellType::Flipendo   => FLIPENDO_COST,
                SpellType::Accio      => ACCIO_COST,
                SpellType::Obliviate  => OBLIVIATE_COST,
                SpellType::Petrificus => PETRIFICUS_COST
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Stats {
        pub collisions: Vec<(EntityId, EntityId)>,
        pub collected: Vec<(EntityId, EntityId)>,
        pub my_score_gain: i64,
        pub enemy_score_gain: i64
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Collision {
        None,
        Entity(EntityId, EntityId),
        WallRebound(EntityId, bool),
        PoleRebound(EntityId, PoleId)
    }
    
    #[derive(Clone, Debug)]
    pub struct TurnState {
        pub round: usize,
        pub winner: Option<Faction>,
        pub my_id: PlayerId,
        pub my_mp: i64,
        pub enemy_mp: i64,
        pub my_score: i64,
        pub enemy_score: i64,
        pub active_spells: Vec<Spell>,
        pub my_goal: Goal,
        pub enemy_goal: Goal,
        pub entities: EntityStore
    }
    
    impl TurnState {
        #[inline]
        pub fn new(my_id: PlayerId) -> TurnState {
            let enemy_id = if my_id == 0 { 1 }
                                    else { 0 };
            TurnState {
                round: 0,
                winner: None,
                my_id: my_id,
                my_mp: 0,
                enemy_mp: 0,
                my_score: 0,
                enemy_score: 0,
                active_spells: Vec::new(),
                entities: EntityStore::new(my_id),
                my_goal: Goal::from_id(my_id),
                enemy_goal: Goal::from_id(enemy_id)
            }
        }

        #[inline]
        pub fn init(entity_count: usize) -> TurnState {
            let mut entities = EntityStore::new(0);
            entities.prepare(entity_count);
            TurnState {
                round: 0,
                winner: None,
                my_id: 0,
                my_mp: 0,
                enemy_mp: 0,
                my_score: 0,
                enemy_score: 0,
                entities: entities,
                active_spells: Vec::new(),
                my_goal: Goal::from_id(0),
                enemy_goal: Goal::from_id(1)
            }
        }

        #[inline]
        pub fn from_vecs(wizards: Vec<Entity>, snaffles: Vec<Entity>, bludgers: Vec<Entity>) -> TurnState {
            let mut inner = Vec::with_capacity(wizards.len() + snaffles.len() + bludgers.len());
            for wizard  in wizards  { inner.push(wizard);  }
            for snaffle in snaffles { inner.push(snaffle); }
            for bludger in bludgers { inner.push(bludger); }
            TurnState {
                round: 0,
                winner: None,
                my_id: 0,
                my_mp: 0,
                enemy_mp: 0,
                my_score: 0,
                enemy_score: 0,
                active_spells: Vec::new(),
                entities: EntityStore {
                    my_id: 0,
                    inner: inner
                },
                my_goal: Goal::from_id(0),
                enemy_goal: Goal::from_id(1)
            }
        }

        pub fn invert(&mut self) {
            self.winner = self.winner.map(|w| w.invert());
            for i in 0..TOTAL_WIZARD_COUNT {
                self.entities[i].faction = self.entities[i].faction.invert();
            }
            mem::swap(&mut self.my_goal, &mut self.enemy_goal);
            mem::swap(&mut self.my_score, &mut self.enemy_score);
            mem::swap(&mut self.my_mp, &mut self.enemy_mp);
            self.my_id = if self.my_id == 0 { 1 } else { 0 };
            self.entities.my_id = self.my_id;
        }

        pub fn draw(&self) {
            draw::set_stroke_color(255, 255, 255, 255);
            draw::set_stroke_width(1.);
            draw::line(Vec2(BORDER_LEFT_X, BORDER_TOP_Y), Vec2(BORDER_RIGHT_X, BORDER_TOP_Y));
            draw::line(Vec2(BORDER_LEFT_X, BORDER_BOTTOM_Y), Vec2(BORDER_RIGHT_X, BORDER_BOTTOM_Y));
            draw::line(Vec2(BORDER_LEFT_X, BORDER_TOP_Y), Vec2(BORDER_LEFT_X, GOAL_Y - GOAL_RADIUS));
            draw::line(Vec2(BORDER_RIGHT_X, BORDER_TOP_Y), Vec2(BORDER_RIGHT_X, GOAL_Y - GOAL_RADIUS));
            draw::line(Vec2(BORDER_LEFT_X, BORDER_BOTTOM_Y), Vec2(BORDER_LEFT_X, GOAL_Y + GOAL_RADIUS));
            draw::line(Vec2(BORDER_RIGHT_X, BORDER_BOTTOM_Y), Vec2(BORDER_RIGHT_X, GOAL_Y + GOAL_RADIUS));
            for &pole in &POLE_POS {
                draw::no_stroke();
                draw::set_fill_color(100, 100, 255, 255);
                draw::circle(pole, POLE_RADIUS);
            }
            for e in self.entities.iter() {
                if !e.dead {
                    e.draw();
                }
            }
        }
        
        #[inline]
        pub fn prepare(&mut self, entity_count: usize) {
            self.entities.prepare(entity_count);
        }
        
        #[inline]
        pub fn my_wizard_ids(&self) -> Range<EntityId> {
            self.entities.my_wizard_ids()
        }
        
        #[inline]
        pub fn enemy_wizard_ids(&self) -> Range<EntityId> {
            self.entities.enemy_wizard_ids()
        }
        
        #[inline]
        pub fn snaffle_ids(&self) -> Range<EntityId> {
            self.entities.snaffle_ids()
        }
        
        #[inline]
        pub fn bludger_ids(&self) -> Range<EntityId> {
            self.entities.bludger_ids()
        }
        
        #[inline]
        pub fn my_wizards(&self) -> &[Entity] {
            self.entities.my_wizards()
        }
        
        #[inline]
        pub fn enemy_wizards(&self) -> &[Entity] {
            self.entities.enemy_wizards()
        }
        
        #[inline]
        pub fn snaffles(&self) -> &[Entity] {
            self.entities.snaffles()
        }
        
        #[inline]
        pub fn bludgers(&self) -> &[Entity] {
            self.entities.bludgers()
        }
        
        #[inline]
        pub fn my_wizards_mut(&mut self) -> &mut [Entity] {
            self.entities.my_wizards_mut()
        }
        
        #[inline]
        pub fn enemy_wizards_mut(&mut self) -> &mut [Entity] {
            self.entities.enemy_wizards_mut()
        }
        
        #[inline]
        pub fn snaffles_mut(&mut self) -> &mut [Entity] {
            self.entities.snaffles_mut()
        }
        
        #[inline]
        pub fn bludgers_mut(&mut self) -> &mut [Entity] {
            self.entities.bludgers_mut()
        }
        
        pub fn simulate(&mut self, my_actions: &[Action], enemy_actions: &[Action]) -> Stats {
            if self.winner.is_some() {
                panic!("Called simulate on a game that has ended!");
            }
            // Stats
            let mut stats = Stats {
                collisions: Vec::with_capacity(8),
                collected: Vec::with_capacity(4),
                my_score_gain: 0,
                enemy_score_gain: 0
            };
            // Snaffle timeouts.
            for i in 0..TOTAL_WIZARD_COUNT {
                self.entities[i].snaffle_timeout -= 1;
            }
            // Stage 1: Thrust
            let mut thrown_snaffles: Vec<(EntityId, Vec2, Vec2, Vec2, i64)> = Vec::with_capacity(4);
            for i in self.my_wizard_ids() {
                let wizard = &mut self.entities[i];
                let my_action = my_actions[wizard.sub_id];
                if let Action::Move(tg, thrust) = my_action {
                    if let Some(sid) = wizard.linked {
                        wizard.linked = None;
                        thrown_snaffles.push((sid, wizard.pos, wizard.vel, Vec2::zero(), 0));
                    }
                    wizard.apply_thrust_towards(tg, thrust as f64);
                }
                else if let Action::Throw(tg, thrust) = my_action {
                    if let Some(sid) = wizard.linked {
                        wizard.linked = None;
                        thrown_snaffles.push((sid, wizard.pos, wizard.vel, tg, thrust));
                    }
                }
                else { // Casting a spell, drop snaffle
                    if let Some(sid) = wizard.linked {
                        wizard.linked = None;
                        thrown_snaffles.push((sid, wizard.pos, wizard.vel, Vec2::zero(), 0));
                    }
                }
            }
            for i in self.enemy_wizard_ids() {
                let wizard = &mut self.entities[i];
                let enemy_action = enemy_actions[wizard.sub_id];
                if let Action::Move(tg, thrust) = enemy_action {
                    if let Some(sid) = wizard.linked {
                        wizard.linked = None;
                        thrown_snaffles.push((sid, wizard.pos, wizard.vel, Vec2::zero(), 0));
                    }
                    wizard.apply_thrust_towards(tg, thrust as f64);
                }
                else if let Action::Throw(tg, thrust) = enemy_action {
                    if let Some(sid) = wizard.linked {
                        wizard.linked = None;
                        thrown_snaffles.push((sid, wizard.pos, wizard.vel, tg, thrust));
                    }
                }
                else { // Casting a spell, drop snaffle
                    if let Some(sid) = wizard.linked {
                        wizard.linked = None;
                        thrown_snaffles.push((sid, wizard.pos, wizard.vel, Vec2::zero(), 0));
                    }
                }
            }
            for (sid, pos, vel, tg, thrust) in thrown_snaffles {
                let snaffle = &mut self.entities[sid];
                snaffle.vel = vel;
                snaffle.pos = pos;
                snaffle.linked = None;
                snaffle.apply_thrust_towards(tg, thrust as f64);
            }
            for i in self.bludger_ids() {
                let mut closest = None;
                for wid in 0..TOTAL_WIZARD_COUNT {
                    if self.entities[i].linked == Some(wid) {
                        continue;
                    }
                    let dist = self.entities[wid].pos.distance_to(self.entities[i].pos);
                    closest = Some(match closest {
                        Some((_, best_dist)) if dist < best_dist => (wid, dist),
                        Some(x) => x,
                        None => (wid, dist)
                    });
                }
                let best_wid = closest.unwrap().0;
                let tg = self.entities[best_wid].pos;
                self.entities[i].apply_thrust_towards(tg, 1000.);
            }
            // TODO: thrust caps (though really my AI never uses too high thrusts)
            // Stage 2: Spells
            for s in &mut self.active_spells {
                match s.kind {
                    SpellType::Flipendo => {
                        let towards = self.entities[s.target_id].pos - self.entities[s.caster_id].pos;
                        let dist = towards.len();
                        if dist == 0. {
                            continue;
                        }
                        let dir = towards.norm();
                        let mut power = 6000. / ( dist / 1000. ).powi(2);
                        if power > 1000. {
                            power = 1000.;
                        }
                        self.entities[s.target_id].apply_thrust(dir * power);
                    },
                    SpellType::Accio => {
                        let towards = self.entities[s.target_id].pos - self.entities[s.caster_id].pos;
                        let dist = towards.len();
                        if dist == 0. {
                            s.casted = 0; // it's fine to do this because spells can't be casted so early
                            continue;
                        }
                        let dir = towards.norm();
                        let mut power = 3000. / ( dist / 1000. ).powi(2);
                        if power > 1000. {
                            power = 1000.;
                        }
                        self.entities[s.target_id].apply_thrust(-dir * power);
                    },
                    SpellType::Petrificus => {
                        self.entities[s.target_id].vel = Vec2::zero();
                    },
                    _ => ()
                }
            }
            let round = self.round;
            self.active_spells.retain(|s| round - s.casted < s.duration);
            // TODO: obliviate
            // Stage 3: Movement
            let mut t = 0.;
            while t < 1. {
                let mut earliest_collision_time = 1. - t;
                let mut collision = Collision::None;
                for i in 0..self.entities.len() {
                    if self.entities[i].dead {
                        continue;
                    }
                    if let Some((col_t, is_vertical)) = self.rebound_time(i) {
                        if col_t < earliest_collision_time {
                            earliest_collision_time = col_t;
                            collision = Collision::WallRebound(i, is_vertical);
                        }
                    }
                    if let Some((col_t, pole_id)) = self.pole_rebound_time(i) {
                        if col_t < earliest_collision_time {
                            earliest_collision_time = col_t;
                            collision = Collision::PoleRebound(i, pole_id);
                        }
                    }
                    for j in (i + 1)..self.entities.len() {
                        if self.entities[j].dead {
                            continue;
                        }
                        if let Some(col_t) = self.collision_time(i, j) {
                            if col_t < earliest_collision_time {
                                earliest_collision_time = col_t;
                                collision = Collision::Entity(i, j);
                            }
                        }
                    }
                }
                for k in 0..self.entities.len() {
                    self.fast_forward(k, earliest_collision_time);
                }
                t += earliest_collision_time;
                if VERBOSE {
                    debug!("[{}] {:?}", t, collision);
                }
                match collision {
                    Collision::WallRebound(i, is_vertical) => {
                        let vel = self.entities[i].vel;
                        if is_vertical {
                            self.entities[i].vel = Vec2(vel.0, -vel.1);
                        }
                        else {
                            self.entities[i].vel = Vec2(-vel.0, vel.1);
                        }
                    },
                    Collision::PoleRebound(i, pid) => {
                        self.pole_rebound_response(i, pid);
                    },
                    Collision::Entity(i, j) => {
                        stats.collisions.push((i, j));
                        self.collision_response(&mut stats, i, j);
                    },
                    Collision::None => ()
                }
            }
            // Stage 4: Friction
            for e in self.entities.iter_mut() {
                e.vel = e.vel * e.kind.friction();
            }
            // Stage 5: Rounding
            for e in self.entities.iter_mut() {
                e.vel = e.vel.round();
                e.pos = e.pos.round();
            }
            // Goals
            for sid in self.snaffle_ids() {
                let snaffle = &mut self.entities[sid];
                if snaffle.dead { continue; }
                if snaffle.pos.0 >= 16000. {
                    snaffle.dead = true;
                    if self.my_id == 0 {
                        self.my_score += 1;
                        stats.my_score_gain += 1;
                    }
                    else {
                        self.enemy_score += 1;
                        stats.enemy_score_gain += 1;
                    }
                }
                else if snaffle.pos.0 <= 0. {
                    snaffle.dead = true;
                    if self.my_id == 0 {
                        self.enemy_score += 1;
                        stats.enemy_score_gain += 1;
                    }
                    else {
                        self.my_score += 1;
                        stats.my_score_gain += 1;
                    }
                }
            }
            // Finalizing snaffle pickups.
            for &(wid, sid) in &stats.collected {
                self.entities[sid].pos = self.entities[wid].pos;
                self.entities[sid].vel = self.entities[wid].vel;
            }
            // Spell casting
            for i in self.my_wizard_ids() {
                let wizard = &mut self.entities[i];
                if let Action::Spell(kind, eid) = my_actions[wizard.sub_id] {
                    if self.my_mp >= kind.cost() {
                        let spell = Spell::new(kind, wizard.id, eid, self.round);
                        if kind == SpellType::Petrificus {
                            self.active_spells.insert(0, spell);
                        }
                        else {
                            self.active_spells.push(spell);
                        }
                        self.my_mp -= kind.cost();
                    }
                }
            }
            for i in self.enemy_wizard_ids() {
                let wizard = &mut self.entities[i];
                if let Action::Spell(kind, eid) = enemy_actions[wizard.sub_id] {
                    if self.enemy_mp >= kind.cost() {
                        let spell = Spell::new(kind, wizard.id, eid, self.round);
                        if kind == SpellType::Petrificus {
                            self.active_spells.insert(0, spell);
                        }
                        else {
                            self.active_spells.push(spell);
                        }
                        self.enemy_mp -= kind.cost();
                    }
                }
            }
            if self.round >= 200 {
                if self.my_score > self.enemy_score {
                    self.winner = Some(Faction::Me);
                }
                else if self.my_score < self.enemy_score {
                    self.winner = Some(Faction::Enemy);
                }
                else {
                    self.winner = Some(Faction::Neutral);
                }
                return stats;
            }
            let snaffle_count = self.entities.len() - TOTAL_WIZARD_COUNT - BLUDGER_COUNT;
            if self.my_score > snaffle_count as i64 / 2 {
                self.winner = Some(Faction::Me);
                return stats;
            }
            if self.enemy_score > snaffle_count as i64 / 2 {
                self.winner = Some(Faction::Enemy);
                return stats;
            }
            self.my_mp += 1;
            self.enemy_mp += 1;
            self.round += 1;
            stats
        }

        pub fn pole_rebound_time(&self, id: EntityId) -> Option<(f64, PoleId)> {
            let ent = &self.entities[id];
            let radius_sum = POLE_RADIUS + ent.kind.radius();
            let mut res: Option<(f64, PoleId)> = None;
            for new_pid in 0..POLE_POS.len() {
                if let Some(new_t) = circle_collision_time(POLE_POS[new_pid] - ent.pos, -ent.vel, radius_sum) {
                    if let Some((best_t, best_pid)) = res {
                        if new_t < best_t {
                            res = Some((new_t, new_pid));
                        }
                    }
                    else {
                        res = Some((new_t, new_pid));
                    }
                }
            }
            res
        }

        pub fn pole_rebound_response(&mut self, id: EntityId, pid: PoleId) {
            let ent = &mut self.entities[id];
            let mass = ent.kind.mass();
            let radius_sum = POLE_RADIUS + ent.kind.radius();
            let dp = POLE_POS[pid] - ent.pos;
            let dv = -ent.vel;
            let raw_impulse = dp * (dp * dv) * (1. / radius_sum.powi(2));
            ent.vel += raw_impulse;
            let mut impulse = if raw_impulse.len() < 100. { raw_impulse.norm() * 100. } else { raw_impulse };
            if impulse.is_nan() {
                impulse = Vec2(0., 0.);
            }
            ent.vel += impulse;
        }
        
        pub fn rebound_time(&self, id: EntityId) -> Option<(f64, bool)> {
            let ent = &self.entities[id];
            let rad = ent.kind.radius();
            let mut t_top = f64::INFINITY;
            let mut t_bottom = f64::INFINITY;
            if ent.vel.1 < 0. {
                t_top = ((BORDER_TOP_Y + rad) - ent.pos.1) / ent.vel.1;
            }
            else if ent.vel.1 > 0. {
                t_bottom = ((BORDER_BOTTOM_Y - rad) - ent.pos.1) / ent.vel.1;
            }
            let mut t_left = f64::INFINITY;
            let mut t_right = f64::INFINITY;
            if ent.vel.0 < 0. {
                t_left = ((BORDER_LEFT_X + rad) - ent.pos.0) / ent.vel.0;
            }
            else if ent.vel.0 > 0. {
                t_right = ((BORDER_RIGHT_X - rad) - ent.pos.0) / ent.vel.0;
            }
            let mut t = t_top;
            let mut is_vertical = true;
            if t_bottom < t { t = t_bottom; }
            if t_left < t { t = t_left; is_vertical = false; }
            if t_right < t { t = t_right; is_vertical = false; }
            if t < f64::INFINITY {
                if !is_vertical {
                    let res_y = ent.pos.1 + t * ent.vel.1;
                    if ent.is_a(EntityType::Snaffle) && res_y > GOAL_Y - GOAL_RADIUS && res_y < GOAL_Y + GOAL_RADIUS {
                        None
                    }
                    else {
                        Some((t, is_vertical))
                    }
                }
                else {
                    Some((t, is_vertical))
                }
            }
            else {
                None
            }
        }
        
        pub fn collision_time(&self, eaid: EntityId, ebid: EntityId) -> Option<f64> {
            if eaid == ebid { panic!("Cannot calculate a collision time with an entity and itself!"); }
            let ea = &self.entities[eaid];
            let eb = &self.entities[ebid];
            let mut radius_sum = ea.kind.radius() + eb.kind.radius();
            // ea can never be a snaffle when eb is a wizard.
            if ea.is_a(EntityType::Wizard) && eb.is_a(EntityType::Snaffle) {
                if ea.snaffle_timeout > 0 || eb.linked.is_some() {
                    return None;
                }
                radius_sum = ea.kind.radius() - 1.;
            }
            let dpos = eb.pos - ea.pos;
            let dvel = eb.vel - ea.vel;
            circle_collision_time(dpos, dvel, radius_sum)
        }
        
        pub fn fast_forward(&mut self, id: EntityId, t: f64) {
            if t > 0. {
                let e = &mut self.entities[id];
                e.pos += e.vel * t;
            }
        }
        
        pub fn collision_response(&mut self, stats: &mut Stats, eaid: EntityId, ebid: EntityId) {
            if eaid == ebid { panic!("Cannot do a collision response with an entity and itself!"); }
            if eaid >= self.entities.len() { panic!("Index out of bounds!"); }
            if ebid >= self.entities.len() { panic!("Index out of bounds!"); }
            let ea: &mut Entity = unsafe { mem::transmute(self.entities.as_mut_ptr().offset(eaid as isize)) };
            let eb: &mut Entity = unsafe { mem::transmute(self.entities.as_mut_ptr().offset(ebid as isize)) };
            if ea.is_a(EntityType::Wizard) {
                if eb.is_a(EntityType::Bludger) { // Wizards come before bludgers.
                    eb.linked = Some(eaid);
                }
                else if eb.is_a(EntityType::Snaffle) { // Wizards also come before snaffles.
                    // These do a different kind of collision resolution. (snaffles get picked up)
                    ea.linked = Some(ebid);
                    eb.linked = Some(eaid);
                    ea.snaffle_timeout = 3;
                    stats.collected.push((eaid, ebid));
                    return;
                }
            }
            let ea_mass = ea.kind.mass();
            let eb_mass = eb.kind.mass();
            let radius_sum = ea.kind.radius() + eb.kind.radius();
            let dp = eb.pos - ea.pos;
            let dv = eb.vel - ea.vel;
            let raw_impulse = dp * (dp * dv) * ea_mass * eb_mass * (1. / (ea_mass + eb_mass) / radius_sum.powi(2));
            ea.vel += raw_impulse * (1. / ea_mass);
            eb.vel -= raw_impulse * (1. / eb_mass);
            let mut impulse = if raw_impulse.len() < 100. { raw_impulse.norm() * 100. } else { raw_impulse };
            if impulse.is_nan() {
                impulse = Vec2(0., 0.);
            }
            ea.vel += impulse * (1. / ea_mass);
            eb.vel -= impulse * (1. / eb_mass);
        }
    }
    
    #[derive(Clone, Debug)]
    pub struct EntityStore {
        inner: Vec<Entity>,
        my_id: PlayerId
    }
    
    impl Deref for EntityStore {
        type Target = [Entity];
        
        #[inline]
        fn deref(&self) -> &[Entity] {
            &self.inner
        }
    }
    
    impl DerefMut for EntityStore {
        #[inline]
        fn deref_mut(&mut self) -> &mut [Entity] {
            &mut self.inner
        }
    }
    
    impl EntityStore {
        pub fn new(my_id: PlayerId) -> EntityStore {
            EntityStore {
                inner: Vec::with_capacity(MAX_ENTITY_COUNT),
                my_id: my_id
            }
        }
        
        pub fn prepare(&mut self, entity_count: usize) {
            if self.inner.is_empty() {
                let mut i = 0;
                for j in 0..WIZARD_COUNT * 2 {
                    let faction = if self.my_id == 0 { if i < WIZARD_COUNT { Faction::Me    } else { Faction::Enemy } }
                                                else { if i < WIZARD_COUNT { Faction::Enemy } else { Faction::Me    } };
                    self.inner.push(Entity::new_wizard(i, j % WIZARD_COUNT, WIZARD_POS[i], Vec2::zero(), faction, None));
                    i += 1;
                }
                for j in 0..entity_count - WIZARD_COUNT * 2 - BLUDGER_COUNT {
                    self.inner.push(Entity::new_snaffle(i, j, Vec2::zero(), Vec2::zero()));
                    i += 1;
                }
                for j in 0..BLUDGER_COUNT {
                    self.inner.push(Entity::new_bludger(i, j, BLUDGER_POS[j], Vec2::zero()));
                    i += 1;
                }
            }
            for e in self.inner.iter_mut() {
                e.dead = true;
            }
        }
        
        #[inline]
        pub fn my_wizards(&self) -> &[Entity] {
            &self.inner[self.my_wizard_ids()]
        }
        
        #[inline]
        pub fn enemy_wizards(&self) -> &[Entity] {
            &self.inner[self.enemy_wizard_ids()]
        }
        
        #[inline]
        pub fn snaffles(&self) -> &[Entity] {
            &self.inner[self.snaffle_ids()]
        }
        
        #[inline]
        pub fn bludgers(&self) -> &[Entity] {
            &self.inner[self.bludger_ids()]
        }
        
        #[inline]
        pub fn my_wizards_mut(&mut self) -> &mut [Entity] {
            let range = self.my_wizard_ids();
            &mut self.inner[range]
        }
        
        #[inline]
        pub fn enemy_wizards_mut(&mut self) -> &mut [Entity] {
            let range = self.enemy_wizard_ids();
            &mut self.inner[range]
        }
        
        #[inline]
        pub fn snaffles_mut(&mut self) -> &mut [Entity] {
            let range = self.snaffle_ids();
            &mut self.inner[range]
        }
        
        #[inline]
        pub fn bludgers_mut(&mut self) -> &mut [Entity] {
            let range = self.bludger_ids();
            &mut self.inner[range]
        }
        
        #[inline]
        pub fn my_wizard_ids(&self) -> Range<EntityId> {
            if self.my_id == 0 {
                0..WIZARD_COUNT
            }
            else {
                WIZARD_COUNT..TOTAL_WIZARD_COUNT
            }
        }
        
        #[inline]
        pub fn enemy_wizard_ids(&self) -> Range<EntityId> {
            if self.my_id == 1 {
                0..WIZARD_COUNT
            }
            else {
                WIZARD_COUNT..TOTAL_WIZARD_COUNT
            }
        }
        
        #[inline]
        pub fn snaffle_ids(&self) -> Range<EntityId> {
            TOTAL_WIZARD_COUNT..self.inner.len() - 2
        }
        
        #[inline]
        pub fn bludger_ids(&self) -> Range<EntityId> {
            self.inner.len() - 2..self.inner.len()
        }
    }
    
    #[derive(Clone, Debug)]
    pub struct Entity {
        pub id: EntityId,
        pub sub_id: SubId,
        pub kind: EntityType,
        pub pos: Vec2,
        pub vel: Vec2,
        pub faction: Faction,
        pub linked: Option<EntityId>,
        pub snaffle_timeout: i64,
        pub dead: bool
    }
    
    impl Entity {
        pub fn new_wizard(id: EntityId, sub_id: SubId, pos: Vec2, vel: Vec2, faction: Faction, snaffle: Option<EntityId>) -> Entity {
            Entity {
                id: id,
                sub_id: sub_id,
                kind: EntityType::Wizard,
                pos: pos,
                vel: vel,
                faction: faction,
                snaffle_timeout: if snaffle.is_some() { 3 } else { 0 },
                linked: snaffle,
                dead: false
            }
        }
        
        pub fn new_snaffle(id: EntityId, sub_id: SubId, pos: Vec2, vel: Vec2) -> Entity {
            Entity {
                id: id,
                sub_id: sub_id,
                kind: EntityType::Snaffle,
                pos: pos,
                vel: vel,
                faction: Faction::Neutral,
                snaffle_timeout: 0,
                linked: None,
                dead: false
            }
        }
        
        pub fn new_bludger(id: EntityId, sub_id: SubId, pos: Vec2, vel: Vec2) -> Entity {
            Entity {
                id: id,
                sub_id: sub_id,
                kind: EntityType::Bludger,
                pos: pos,
                vel: vel,
                faction: Faction::Neutral,
                snaffle_timeout: 0,
                linked: None,
                dead: false
            }
        }
        
        #[inline]
        pub fn is_a(&self, kind: EntityType) -> bool {
            self.kind == kind
        }
        
        #[inline]
        pub fn aligned_with(&self, faction: Faction) -> bool {
            self.faction == faction
        }
        
        pub fn draw(&self) {
            draw::no_stroke();
            match self.kind {
                EntityType::Wizard  => if self.aligned_with(Faction::Me){
                    draw::set_fill_color(255, 255, 0, 255);
                } else {
                    draw::set_fill_color(255, 0, 0, 255);
                },
                EntityType::Snaffle => draw::set_fill_color(255, 99, 255, 255),
                EntityType::Bludger => draw::set_fill_color(0, 255, 255, 255)
            }
            draw::circle(self.pos, self.kind.radius());
        }
        
        pub fn apply_thrust(&mut self, thrust: Vec2) {
            self.vel += thrust * (1. / self.kind.mass());
        }
        
        pub fn apply_thrust_towards(&mut self, towards: Vec2, thrust: f64) {
            let norm = (towards - self.pos).norm();
            self.apply_thrust(norm * thrust);
        }
    }
    
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Action {
        Move(Vec2, i64),
        Throw(Vec2, i64),
        Spell(SpellType, EntityId)
    }
    
    impl ToString for Action {
        fn to_string(&self) -> String {
            match *self {
                Action::Move(tg, thrust)                 => format!("MOVE {} {} {}", tg.0 as i64, tg.1 as i64, thrust),
                Action::Throw(tg, thrust)                => format!("THROW {} {} {}", tg.0 as i64, tg.1 as i64, thrust),
                Action::Spell(SpellType::Obliviate, id)  => format!("OBLIVIATE {}", id),
                Action::Spell(SpellType::Petrificus, id) => format!("PETRIFICUS {}", id),
                Action::Spell(SpellType::Accio, id)      => format!("ACCIO {}", id),
                Action::Spell(SpellType::Flipendo, id)   => format!("FLIPENDO {}", id)
            }
        }
    }

    impl FromStr for Action {
        type Err = ();
        fn from_str(s: &str) -> Result<Action, ()> {
            let mut sp = s.split(" ");
            let cmd = sp.next().unwrap();
            Ok(match cmd {
                "MOVE" => {
                    let x: f64 = sp.next().unwrap().parse().unwrap();
                    let y: f64 = sp.next().unwrap().parse().unwrap();
                    let thrust: i64 = sp.next().unwrap().parse().unwrap();
                    Action::Move(Vec2(x, y), thrust)
                },
                "THROW" => {
                    let x: f64 = sp.next().unwrap().parse().unwrap();
                    let y: f64 = sp.next().unwrap().parse().unwrap();
                    let thrust: i64 = sp.next().unwrap().parse().unwrap();
                    Action::Throw(Vec2(x, y), thrust)
                },
                "OBLIVIATE" => {
                    let eid: EntityId = sp.next().unwrap().parse().unwrap();
                    Action::Spell(SpellType::Obliviate, eid)
                },
                "PETRIFICUS" => {
                    let eid: EntityId = sp.next().unwrap().parse().unwrap();
                    Action::Spell(SpellType::Petrificus, eid)
                },
                "ACCIO" => {
                    let eid: EntityId = sp.next().unwrap().parse().unwrap();
                    Action::Spell(SpellType::Accio, eid)
                },
                "FLIPENDO" => {
                    let eid: EntityId = sp.next().unwrap().parse().unwrap();
                    Action::Spell(SpellType::Flipendo, eid)
                },
                _ => return Err(())
            })
        }
    }
    
    #[derive(Clone)]
    pub struct Goal {
        pub center: Vec2
    }
    
    impl fmt::Debug for Goal {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(fmt, "Goal({:?})", self.center)
        }
    }
    
    impl Goal {
        pub fn from_id(id: PlayerId) -> Goal {
            Goal {
                center: GOAL_POSITIONS[id].clone()
            }
        }

        pub fn distance_to(&self, tg: Vec2) -> f64 {
            let mut dist = 0.;
            if tg.1 < GOAL_Y - GOAL_RADIUS * 0.7 { // This coefficient to stay safe.
                dist += (GOAL_Y - GOAL_RADIUS - tg.1).powi(2);
            }
            else if tg.1 > GOAL_Y + GOAL_RADIUS * 0.7 {
                dist += (tg.1 - GOAL_Y - GOAL_RADIUS).powi(2);
            }
            (dist + (self.center.0 - tg.0).powi(2)).sqrt()
        }
    }
}
mod ai { /* … super sekrit … */ }
