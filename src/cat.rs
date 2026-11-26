//! What Helvilette does: moods, paws, eyes, and how he reacts to the user.

use crate::art::{EAR_TIPS, FACE, KEYBOARD, MOUSE_REST, MOUTH, P, PAW_REST, Pose, RUNES};

/// Seconds without input before the cat becomes drowsy, then falls asleep.
/// The zsh plugin uses the same values.
pub const DROWSY_AFTER: f32 = 30.0;
pub const ASLEEP_AFTER: f32 = 90.0;

/// Short name for the tray, full name for the meal.
pub const FOODS: [(&str, &str); 5] = [
    ("Bánh cam", "bánh cam nhân đậu xanh"),
    ("Tai yến", "bánh tai yến"),
    ("Chè dừa", "chè nước cốt dừa"),
    ("Mỡ cá kho", "mỡ cá kho tộ"),
    ("Bánh cuốn", "bánh cuốn chấm nước mắm"),
];

/// Moods. Some belong to one of the twelve True Selves (see `Cat::rune`).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mood {
    Awake,
    Drowsy,
    Asleep,
    Startled,
    Petted,
    Chin,
    Eating,
    /// Hagalaz: woken up, he grumbles.
    Grumpy,
    /// Sowilo: petted for too long, he swipes at the pointer.
    Swipe,
    /// Laguz: hungry and sad.
    Sad,
}

/// Indices in `RUNES`.
const OTHELA: usize = 0;
const ISA: usize = 4;
const WUNJO: usize = 5;
const LAGUZ: usize = 6;
const HAGALAZ: usize = 7;
const SOWILO: usize = 9;

const PET_HEAD: (P, P) = ((1390.0, 650.0), (150.0, 105.0));
const PET_CHIN: (P, P) = ((MOUTH.0, 815.0), (75.0, 40.0));
/// Pointer travel (artwork pixels) over the head before the cat purrs.
const PET_NEEDED: f32 = 500.0;
/// More petting while he purrs, before he has had enough and swipes.
const PET_TOO_MUCH: f32 = 1250.0;
/// Seconds without focus before the terminal counts as unobserved.
const UNSEEN_AFTER: f32 = 2.0;
/// Length of the Voided Out darkness.
const VOID_SECS: f32 = 0.9;

pub struct Cat {
    pub mood: Mood,
    pub food: Option<usize>,
    t: f32,
    idle: f32,
    mood_left: f32,
    typed: f32,
    paw_up: [f32; 2],
    next_paw: usize,
    key: Option<(f32, f32)>,
    mouse: P,
    on_mouse: f32,
    pointer: Option<(P, f32)>,
    pet: f32,
    chin: f32,
    overpet: f32,
    no_pet: f32,
    hungry: bool,
    /// The True Self that is present when no mood names another one.
    persona: usize,
    unseen: Option<f32>,
    void: f32,
    introduce: f32,
    ear: [f32; 2],
    blink: (f32, f32, f32),
    glitch: Option<(f32, i32, f32)>,
    next_glitch: f32,
    rng: u32,
}

impl Cat {
    pub fn new(seed: u32) -> Self {
        Cat {
            mood: Mood::Awake,
            food: None,
            t: 0.0,
            idle: 0.0,
            mood_left: 0.0,
            typed: 99.0,
            paw_up: [0.0; 2],
            next_paw: 0,
            key: None,
            mouse: (0.0, 0.0),
            on_mouse: 0.0,
            pointer: None,
            pet: 0.0,
            chin: 0.0,
            overpet: 0.0,
            no_pet: 0.0,
            hungry: false,
            persona: OTHELA,
            unseen: None,
            void: 0.0,
            introduce: 0.0,
            ear: [0.0; 2],
            blink: (2.5, -10.0, 0.18),
            glitch: None,
            next_glitch: 3.0,
            rng: seed | 1,
        }
    }

    /// xorshift32, in 0..1.
    fn rand(&mut self) -> f32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        (self.rng >> 8) as f32 / (1u32 << 24) as f32
    }

    fn wake(&mut self) {
        self.idle = 0.0;
        match self.mood {
            Mood::Asleep => self.set_mood(Mood::Startled, 1.0),
            Mood::Drowsy => self.mood = Mood::Awake,
            _ => {}
        }
    }

    fn set_mood(&mut self, mood: Mood, secs: f32) {
        self.mood = mood;
        self.mood_left = secs;
    }

    /// The user pressed a key: a paw taps the keyboard.
    pub fn key(&mut self) {
        self.wake();
        self.typed = 0.0;
        self.paw_up[self.next_paw] = 0.12;
        self.next_paw ^= 1;
        let at = self.rand();
        self.key = Some((at, 0.15));
    }

    /// The pointer moved to `p` (artwork pixels): the cat's mouse moves, and the
    /// pointer can pet the head, scratch the chin, or touch an ear.
    pub fn pointer(&mut self, p: P) {
        self.wake();
        let from = self.pointer.map_or(p, |(q, _)| q);
        let (dx, dy) = (p.0 - from.0, p.1 - from.1);
        let dist = (dx * dx + dy * dy).sqrt();
        self.pointer = Some((p, 0.0));
        self.mouse.0 = (self.mouse.0 + dx * 0.25).clamp(-45.0, 45.0);
        self.mouse.1 = (self.mouse.1 + dy * 0.25).clamp(-30.0, 30.0);
        self.on_mouse = 1.5;

        if matches!(self.mood, Mood::Eating | Mood::Startled | Mood::Swipe) || self.no_pet > 0.0 {
            return;
        }
        let inside =
            |(c, r): (P, P)| ((p.0 - c.0) / r.0).powi(2) + ((p.1 - c.1) / r.1).powi(2) < 1.0;
        let on_cat = inside(PET_CHIN) || inside(PET_HEAD);
        if on_cat && matches!(self.mood, Mood::Petted | Mood::Chin) {
            self.overpet += dist;
            if self.overpet > PET_TOO_MUCH {
                self.set_mood(Mood::Swipe, 1.2);
                self.no_pet = 6.0;
                return;
            }
        }
        if inside(PET_CHIN) {
            self.chin += dist;
            if self.chin > PET_NEEDED * 0.6 {
                self.set_mood(Mood::Chin, 2.5);
            }
        } else if inside(PET_HEAD) {
            self.pet += dist;
            if self.pet > PET_NEEDED {
                self.set_mood(Mood::Petted, 2.5);
            }
        }
        for (i, tip) in EAR_TIPS.iter().enumerate() {
            if (p.0 - tip.0).hypot(p.1 - tip.1) < 60.0 && self.ear[i] <= 0.0 {
                self.ear[i] = 0.4;
            }
        }
    }

    /// Laguz comes when he is hungry; a meal sends him away.
    pub fn set_hungry(&mut self, hungry: bool) {
        self.hungry = hungry;
    }

    /// Terminal focus. Schrödinger Cat: while nobody looks, he can be any of his
    /// True Selves. When you look again, darkness covers him (Voided Out) and
    /// another True Self is there.
    pub fn focus(&mut self, on: bool) {
        if !on {
            self.unseen.get_or_insert(0.0);
            return;
        }
        if self.unseen.take().is_some_and(|secs| secs >= UNSEEN_AFTER) {
            let other = 1 + (self.rand() * (RUNES.len() - 1) as f32) as usize;
            self.persona = (self.persona + other) % RUNES.len();
            self.void = VOID_SECS;
            self.introduce = VOID_SECS + 2.5;
        }
    }

    /// The True Self whose rune glows now.
    pub fn rune(&self) -> usize {
        match self.mood {
            Mood::Petted | Mood::Chin => WUNJO,
            Mood::Grumpy => HAGALAZ,
            Mood::Swipe => SOWILO,
            Mood::Asleep => ISA,
            // A True Self that just arrived shows its own rune while it says its name.
            Mood::Sad if self.introduce <= 0.0 => LAGUZ,
            _ => self.persona,
        }
    }

    pub fn feed(&mut self, food: usize) {
        self.wake();
        self.food = Some(food);
        self.set_mood(Mood::Eating, 3.0);
    }

    pub fn update(&mut self, dt: f32) {
        self.t += dt;
        self.idle += dt;
        self.typed += dt;
        self.on_mouse -= dt;
        self.paw_up
            .iter_mut()
            .chain(self.ear.iter_mut())
            .for_each(|v| *v -= dt);
        self.key = self
            .key
            .map(|(at, left)| (at, left - dt))
            .filter(|k| k.1 > 0.0);
        self.pointer = self.pointer.map(|(p, age)| (p, age + dt));
        let decay = (-dt * 0.8).exp();
        self.mouse = (self.mouse.0 * decay, self.mouse.1 * decay);
        self.pet *= (-dt * 0.5).exp();
        self.chin *= (-dt * 0.5).exp();
        self.overpet *= (-dt * 0.3).exp();
        self.no_pet -= dt;
        self.void = (self.void - dt).max(0.0);
        self.introduce -= dt;
        if let Some(secs) = &mut self.unseen {
            *secs += dt;
        }

        self.mood_left -= dt;
        match self.mood {
            // A cat that you wake up grumbles before he forgives you.
            Mood::Startled if self.mood_left <= 0.0 => self.set_mood(Mood::Grumpy, 2.5),
            Mood::Petted | Mood::Chin | Mood::Eating | Mood::Grumpy | Mood::Swipe
                if self.mood_left <= 0.0 =>
            {
                self.mood = Mood::Awake;
                self.food = None;
                self.pet = 0.0;
                self.chin = 0.0;
                self.overpet = 0.0;
            }
            Mood::Awake | Mood::Sad if self.idle >= DROWSY_AFTER => self.mood = Mood::Drowsy,
            Mood::Awake if self.hungry => self.mood = Mood::Sad,
            Mood::Sad if !self.hungry => self.mood = Mood::Awake,
            Mood::Drowsy if self.idle >= ASLEEP_AFTER => self.mood = Mood::Asleep,
            _ => {}
        }

        if self.t > self.blink.0 {
            let next = self.t + 3.5 + self.rand() * 4.0;
            self.blink = (next, self.t, 0.18);
        }
        self.glitch = self
            .glitch
            .map(|(y, off, left)| (y, off, left - dt))
            .filter(|g| g.2 > 0.0);
        if self.t > self.next_glitch {
            let y = 450.0 + self.rand() * 950.0;
            let off = ((self.rand() - 0.5) * 8.0).round() as i32;
            self.glitch = Some((y, if off == 0 { 2 } else { off }, 0.08 + self.rand() * 0.15));
            self.next_glitch = self.t + 2.0 + self.rand() * 4.0;
        }
    }

    pub fn pose(&self) -> Pose {
        let mut p = Pose {
            t: self.t,
            ..Pose::default()
        };
        let (_, start, len) = self.blink;
        let bt = (self.t - start) / len;
        let blink = if (0.0..=1.0).contains(&bt) {
            (std::f32::consts::PI * bt).sin()
        } else {
            0.0
        };
        p.lid = blink;

        match self.mood {
            Mood::Awake => {}
            Mood::Drowsy => {
                // Nodding off: the head droops slowly, then jerks up.
                let phase = (self.t * 0.25).fract();
                p.head = (0.0, 30.0 * phase.powi(3));
                p.lid = 0.5 + 0.45 * phase;
            }
            Mood::Asleep => {
                p.head = (0.0, 26.0);
                p.lid = 1.0;
            }
            Mood::Startled => {
                p.lid = 0.0;
                p.pupil = 3.5;
                p.head = (0.0, -6.0);
                p.ears = [(-6.0, -16.0), (6.0, -16.0)];
            }
            Mood::Petted => {
                p.lid = 1.0;
                p.happy = true;
                p.head = (6.0 * (self.t * 2.0).sin(), -4.0);
                p.ears = [(-10.0, 10.0), (10.0, 10.0)];
            }
            Mood::Chin => {
                p.lid = 1.0;
                p.happy = true;
                p.head = (0.0, -16.0);
            }
            Mood::Eating => {
                p.lid = 0.3;
                p.mouth = (self.t * 10.0).sin() * 0.5 + 0.5;
                p.head = (0.0, 14.0);
            }
            Mood::Grumpy => {
                // A flat, sideways look with the ears turned out.
                p.lid = 0.55;
                p.pupil = 6.0;
                p.ears = [(-12.0, 6.0), (12.0, 6.0)];
            }
            Mood::Swipe => {
                p.lid = 0.15;
                p.pupil = 3.5;
                p.mouth = 0.7;
                p.head = (0.0, -6.0);
                p.ears = [(-16.0, 12.0), (16.0, 12.0)];
            }
            Mood::Sad => {
                p.lid = 0.4;
                p.head = (0.0, 12.0);
                p.ears = [(-8.0, 8.0), (8.0, 8.0)];
            }
        }
        for i in 0..2 {
            if self.ear[i] > 0.0 {
                let dir = if i == 0 { -1.0 } else { 1.0 };
                p.ears[i] = (
                    p.ears[i].0 + dir * 8.0 * (self.ear[i] * 60.0).sin(),
                    p.ears[i].1 - 6.0,
                );
            }
        }

        p.mouse = (MOUSE_REST.0 + self.mouse.0, MOUSE_REST.1 + self.mouse.1);
        p.gaze = match self.pointer {
            Some((q, age)) if age < 3.0 => q,
            _ if self.typed < 2.0 => ((KEYBOARD[0] + KEYBOARD[2]) / 2.0, KEYBOARD[1]),
            _ if self.on_mouse > 0.0 => p.mouse,
            _ => FACE,
        };
        p.lamp = self.pointer.filter(|(_, age)| *age < 3.0).map(|(q, _)| q);
        if matches!(self.mood, Mood::Eating | Mood::Sad) {
            p.gaze = (FACE.0, FACE.1 + 300.0);
        }

        p.paws = PAW_REST;
        for i in 0..2 {
            if self.paw_up[i] > 0.0 {
                p.paws[i].1 -= 32.0;
            }
        }
        if self.on_mouse > 0.0 && self.mood != Mood::Asleep {
            // The paw rests on the back of the mouse.
            p.paws[1] = (p.mouse.0 - 4.0, p.mouse.1 - 64.0);
        }
        if self.mood == Mood::Swipe {
            // One fast swipe: the right paw goes up and comes down.
            let s = (1.0 - self.mood_left / 1.2).clamp(0.0, 1.0);
            p.paws[1] = (
                PAW_REST[1].0 + 40.0,
                PAW_REST[1].1 - 150.0 * (std::f32::consts::PI * s).sin(),
            );
        }
        p.key = self.key.map(|(at, _)| at);
        p.glitch = self.glitch.map(|(y, off, _)| (y, off));
        p.rune = Some(self.rune());
        // The darkness falls at once, then the light comes back slowly.
        p.dark = (self.void / VOID_SECS).powi(2);
        p
    }

    /// Words next to the head, if any.
    pub fn words(&self) -> Option<String> {
        let intro = || {
            let (ch, name) = RUNES[self.persona];
            (self.void <= 0.0 && self.introduce > 0.0).then(|| format!("{ch} {name}"))
        };
        Some(match self.mood {
            Mood::Drowsy => "...".into(),
            Mood::Asleep => if (self.t * 0.8).fract() < 0.5 {
                "z Z"
            } else {
                "Z z"
            }
            .into(),
            Mood::Startled => "!".into(),
            Mood::Petted | Mood::Chin => "ᚹ purr~".into(),
            Mood::Eating => format!("nom nom · {}", FOODS[self.food?].1),
            Mood::Grumpy => "ᚺ hmph.".into(),
            Mood::Swipe => "ᛊ Hssss!".into(),
            Mood::Sad if self.introduce <= 0.0 => {
                return ((self.t % 8.0) < 2.5).then(|| "ᛚ meo...".into());
            }
            Mood::Awake | Mood::Sad => return intro(),
        })
    }

    /// Seconds between frames: slower while he sleeps.
    pub fn frame_secs(&self) -> f32 {
        if self.mood == Mood::Asleep { 0.1 } else { 0.05 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_cat_gets_drowsy_then_sleeps_and_wakes_startled() {
        let mut cat = Cat::new(1);
        cat.update(DROWSY_AFTER + 1.0);
        assert_eq!(cat.mood, Mood::Drowsy);
        cat.update(ASLEEP_AFTER - DROWSY_AFTER);
        assert_eq!(cat.mood, Mood::Asleep);
        assert_eq!(cat.pose().lid, 1.0);
        assert_eq!(cat.rune(), ISA);
        cat.key();
        assert_eq!(cat.mood, Mood::Startled);
        cat.update(1.1);
        assert_eq!((cat.mood, cat.rune()), (Mood::Grumpy, HAGALAZ));
        assert_eq!(cat.words().as_deref(), Some("ᚺ hmph."));
        cat.update(2.6);
        assert_eq!(cat.mood, Mood::Awake);
    }

    fn stroke_head(cat: &mut Cat, times: usize) {
        for i in 0..times {
            cat.pointer((if i % 2 == 0 { 1300.0 } else { 1480.0 }, 640.0));
        }
    }

    #[test]
    fn petting_too_long_makes_him_swipe_and_refuse() {
        let mut cat = Cat::new(1);
        stroke_head(&mut cat, 6);
        assert_eq!((cat.mood, cat.rune()), (Mood::Petted, WUNJO));
        stroke_head(&mut cat, 10);
        assert_eq!((cat.mood, cat.rune()), (Mood::Swipe, SOWILO));
        cat.update(0.6);
        assert!(
            cat.pose().paws[1].1 < PAW_REST[1].1 - 100.0,
            "the paw swipes up"
        );
        cat.update(0.7);
        assert_eq!(cat.mood, Mood::Awake);
        stroke_head(&mut cat, 10);
        assert_eq!(cat.mood, Mood::Awake, "he refuses petting for a while");
        cat.update(6.0);
        stroke_head(&mut cat, 6);
        assert_eq!(cat.mood, Mood::Petted);
    }

    #[test]
    fn hunger_makes_him_sad_and_a_meal_fixes_it() {
        let mut cat = Cat::new(1);
        cat.set_hungry(true);
        cat.update(0.1);
        assert_eq!((cat.mood, cat.rune()), (Mood::Sad, LAGUZ));
        cat.feed(0);
        cat.set_hungry(false);
        cat.update(3.1);
        assert_eq!(cat.mood, Mood::Awake);
    }

    #[test]
    fn unobserved_he_comes_back_as_another_true_self() {
        let mut cat = Cat::new(7);
        cat.focus(false);
        cat.update(0.5);
        cat.focus(true);
        assert_eq!(
            (cat.rune(), cat.pose().dark),
            (OTHELA, 0.0),
            "a short look away changes nothing"
        );

        cat.focus(false);
        cat.update(UNSEEN_AFTER + 0.5);
        cat.focus(true);
        let other = cat.rune();
        assert_ne!(other, OTHELA);
        assert!(cat.pose().dark > 0.9);
        cat.update(VOID_SECS + 0.1);
        assert_eq!(cat.pose().dark, 0.0);
        assert_eq!(
            cat.words(),
            Some(format!("{} {}", RUNES[other].0, RUNES[other].1))
        );
    }

    #[test]
    fn keys_lift_the_paws_in_turn() {
        let mut cat = Cat::new(1);
        cat.key();
        let p = cat.pose();
        assert!(p.paws[0].1 < PAW_REST[0].1 && p.paws[1] == PAW_REST[1]);
        cat.update(0.2);
        cat.key();
        let p = cat.pose();
        assert!(p.paws[1].1 < PAW_REST[1].1 && p.paws[0] == PAW_REST[0]);
        assert!(p.key.is_some());
    }

    #[test]
    fn stroking_the_head_makes_him_purr() {
        let mut cat = Cat::new(1);
        stroke_head(&mut cat, 6);
        assert_eq!(cat.mood, Mood::Petted);
        assert!(cat.pose().happy);
        cat.update(3.0);
        assert_eq!(cat.mood, Mood::Awake);
    }

    #[test]
    fn mouse_movement_moves_his_mouse_and_paw() {
        let mut cat = Cat::new(1);
        cat.pointer((1000.0, 1000.0));
        cat.pointer((1100.0, 1000.0));
        let p = cat.pose();
        assert!(p.mouse.0 > MOUSE_REST.0);
        assert!((p.paws[1].0 - p.mouse.0).abs() < 30.0);
    }

    #[test]
    fn feeding_lasts_three_seconds() {
        let mut cat = Cat::new(1);
        cat.feed(4);
        assert_eq!(cat.mood, Mood::Eating);
        assert!(cat.words().unwrap().contains("bánh cuốn"));
        cat.update(3.1);
        assert_eq!((cat.mood, cat.food), (Mood::Awake, None));
    }
}
