

//..#*#..
//...#...
//.......
//.~~.~~.
//.~~.~~.
//.~~.~~.
//.......
//...#...
//..#*#..


#[derive(Copy, Clone)]
struct Pt
{
    x: i8,
    y: i8,
}

impl Pt
{
    fn is_valid(&self) -> bool
    {
       self.x >= 0 && self.x < 7 && self.y >= 0 && self.y < 9
    }

    fn is_water(&self) -> bool
    {
        if !self.is_valid() { return false; }
        if self.x == 0 || self.x == 3 || self.x == 6 { return false; }
        if self.y < 3 || self.y > 5 { return false; }
        return true;
    }

    fn is_trap(&self) -> bool
    {
        if (self.y == 0 || self.y == 8) && (self.x == 2 || self.x == 4) { return true; }
        if (self.y == 1 || self.y == 7) && self.x == 3  { return true; }
        return false;
    }

    fn is_between(&self, l: &Pt, r : &Pt) -> bool
    {
        if self.x == l.x && r.x == self.x
        {
            if self.y > std::cmp::min(l.y,r.y) && self.y < std::cmp::max(l.y,r.y)
            {
                return true;
            }
        }
        else if self.y == l.y && r.y == self.y
        {
            if self.x > std::cmp::min(l.x,r.x) && self.x < std::cmp::max(l.x,r.x)
            {
                return true;
            }
        }
        false
    }

    fn equals(&self, other: &Pt) -> bool
    {
        if self.x == other.x && self.y == other.y
        {
            return true;
        }
        return false;
    }

    fn equals_opt(&self, other: &Option<Pt>) -> bool
    {
        if other.is_some()
        {
            return self.equals(other.as_ref().unwrap());
        }
        return false;
    }

}

fn generate_jump(l : &Pt, w : &Pt) -> Pt
{
    if l.x == w.x
    {
        return Pt { x: l.x, y: l.y + 3*(w.y-l.y)};
    }
    return Pt { x: l.x + 2 * (w.x-l.x), y: l.y};
}

#[derive(Copy, Clone)]
struct Pionki
{
    rat: Option<Pt>,
    cat: Option<Pt>,
    dog: Option<Pt>,
    wolf: Option<Pt>,
    panther: Option<Pt>,
    tiger: Option<Pt>,
    lion: Option<Pt>,
    elephant: Option<Pt>,
    cave: Pt,
}

impl Pionki
{
    fn is_empty_pt(&self, p: &Pt) -> bool
    {
        if self.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }
        if self.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).equals(p) { return false; }

        return true;
    }

    fn eliminate(&mut self, p: &Pt)
    {
        if p.equals_opt(&self.rat)
        {
            self.rat = None;
            return;
        }

        if p.equals_opt(&self.cat)
        {
            self.cat = None;
            return;
        }

        if p.equals_opt(&self.dog)
        {
            self.dog = None;
            return;
        }

        if p.equals_opt(&self.wolf)
        {
            self.wolf = None;
            return;
        }

        if p.equals_opt(&self.panther)
        {
            self.panther = None;
            return;
        }

        if p.equals_opt(&self.tiger)
        {
            self.tiger = None;
            return;
        }

        if p.equals_opt(&self.lion)
        {
            self.lion = None;
            return;
        }

        if p.equals_opt(&self.elephant)
        {
            self.elephant = None;
            return;
        }

    }

}

fn move_generator( m: Pionki, r: Pionki) -> Vec<(i8, i8, i8, i8)>
{
    //It generates all possible, legal moves for first player (m).
    let dirs = [(0,1), (0,-1), (1,0), (-1,0)];
    let mut moves = Vec::new();

    if m.rat.is_some()
    {
        let org_pt = m.rat.as_ref().unwrap();
        for d in dirs
        {
            let new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            // Jeśli nowe pole puste to ok
            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if org_pt.is_water() && !new_pt.is_water() { continue; }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x //
            && new_pt.y == r.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y) //
            || (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x      //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)      //
            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.cat.is_some()
    {
        let org_pt = m.cat.as_ref().unwrap();
        for d in dirs
        {
            let new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x      //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)      //
            || (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x      //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)      //
            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.dog.is_some()
    {
        let org_pt = m.dog.as_ref().unwrap();
        for d in dirs
        {
            let new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.wolf.is_some()
    {
        let org_pt = m.wolf.as_ref().unwrap();
        for d in dirs
        {
            let new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.panther.is_some()
    {
        let org_pt = m.panther.as_ref().unwrap();
        for d in dirs
        {
            let new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.tiger.is_some()
    {
        let org_pt = m.tiger.as_ref().unwrap();
        for d in dirs
        {
            let mut new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.is_water() 
            { 
                new_pt = generate_jump(org_pt, &new_pt);

                if r.rat.as_ref().is_some()
                {
                    if r.rat.as_ref().unwrap().is_between(&new_pt, org_pt)
                    {
                        continue;
                    }
                }
            }

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.lion.is_some()
    {
        let org_pt = m.lion.as_ref().unwrap();
        for d in dirs
        {
            let mut new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.is_water() 
            { 
                new_pt = generate_jump(org_pt, &new_pt);

                if r.rat.as_ref().is_some()
                {
                    if r.rat.as_ref().unwrap().is_between(&new_pt, org_pt)
                    {
                        continue;
                    }
                }
            }

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //

            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    if m.elephant.is_some()
    {
        let org_pt = m.elephant.as_ref().unwrap();
        for d in dirs
        {
            let new_pt = Pt{x: org_pt.x+d.0, y: org_pt.y+d.1};

            if !new_pt.is_valid() { continue; }

            if new_pt.x == m.cave.x && new_pt.y == m.cave.y { continue; }

            if new_pt.x == m.rat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.rat.as_ref().unwrap_or(&Pt{x:-1,y:-1}).y { continue; }
            if new_pt.x == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }
            if new_pt.x == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x && new_pt.y == m.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y { continue; }

            if new_pt.is_water() { continue; }

            if r.is_empty_pt(&new_pt)
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if new_pt.is_trap() 
            {
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue;
            }

            if (new_pt.x == r.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.elephant.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.cat.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.dog.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.wolf.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.panther.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.tiger.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //
            || (new_pt.x == r.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).x    //
            && new_pt.y == r.lion.as_ref().unwrap_or(&Pt{x: -1, y: -1}).y)    //

            { 
                moves.push((org_pt.x, org_pt.y, new_pt.x, new_pt.y));
                continue; 
            }
        }
    }

    moves.push((-1,-1,-1,-1));

    moves
}

fn do_move(mut p1 : Pionki, mut p2 : Pionki, mov : (i8, i8, i8, i8)) -> (Pionki, Pionki)
{
    if mov == (-1, -1, -1, -1)
    {
        return (p1,p2);
    }

    let from = Pt { x: mov.0, y: mov.1};
    let to = Pt { x: mov.2, y: mov.3};

    if !p2.is_empty_pt(&from)
    {
        return (p1,p2);
    }

    if p1.cave.equals(&to)
    {
        return (p1,p2);
    }

    if from.equals_opt(&p1.rat)
    {
        if p2.is_empty_pt(&to)
        {
            p1.rat = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if from.is_water() && !to.is_water()
        {
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.rat = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.elephant)
        {
            p1.rat = Some(Pt { x: to.x, y: to.y });
            p2.elephant = None;
            return (p1,p2);
        }

        if to.is_trap()
        {
            p1.rat = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
        }

    }

    if from.equals_opt(&p1.cat)
    {
        if p2.is_empty_pt(&to)
        {
            p1.cat = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.cat = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.cat = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.cat = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }

    }

    if from.equals_opt(&p1.dog)
    {
        if p2.is_empty_pt(&to)
        {
            p1.dog = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.dog = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.dog = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }


        if to.equals_opt(&p2.dog)
        {
            p1.dog = Some(Pt { x: to.x, y: to.y });
            p2.dog = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.dog = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }

    }

    if from.equals_opt(&p1.wolf)
    {
        if p2.is_empty_pt(&to)
        {
            p1.wolf = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.wolf = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.wolf = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }


        if to.equals_opt(&p2.dog)
        {
            p1.wolf = Some(Pt { x: to.x, y: to.y });
            p2.dog = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.wolf)
        {
            p1.wolf = Some(Pt { x: to.x, y: to.y });
            p2.wolf = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.wolf = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }

    }

    if from.equals_opt(&p1.panther)
    {
        if p2.is_empty_pt(&to)
        {
            p1.panther = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.panther = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.panther = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.dog)
        {
            p1.panther = Some(Pt { x: to.x, y: to.y });
            p2.dog = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.wolf)
        {
            p1.panther = Some(Pt { x: to.x, y: to.y });
            p2.wolf = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.panther)
        {
            p1.panther = Some(Pt { x: to.x, y: to.y });
            p2.panther = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.panther = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }

    }

    if from.equals_opt(&p1.tiger)
    {
        if p2.is_empty_pt(&to)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.dog)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.dog = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.wolf)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.wolf = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.panther)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.panther = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.tiger)
        {
            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.tiger = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.tiger = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }

    }

    if from.equals_opt(&p1.lion)
    {
        if p2.is_empty_pt(&to)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.rat)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.rat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.dog)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.dog = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.wolf)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.wolf = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.panther)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.panther = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.tiger)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.tiger = None;
            return (p1,p2);
        }


        if to.equals_opt(&p2.lion)
        {
            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.lion = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }

    }

    if from.equals_opt(&p1.elephant)
    {
        if p2.is_empty_pt(&to)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            return (p1,p2);
        }

        if to.equals_opt(&p2.elephant)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.elephant = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.cat)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.cat = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.dog)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.dog = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.wolf)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.wolf = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.panther)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.panther = None;
            return (p1,p2);
        }

        if to.equals_opt(&p2.tiger)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.tiger = None;
            return (p1,p2);
        }


        if to.equals_opt(&p2.lion)
        {
            p1.elephant = Some(Pt { x: to.x, y: to.y });
            p2.lion = None;
            return (p1,p2);
        }

        if to.is_trap()
        {

            p1.lion = Some(Pt { x: to.x, y: to.y });
            p2.eliminate(&to);
            return (p1,p2);
        }        

    }

    return (p1,p2);
}

fn agent_losowy(p1 : Pionki, p2 : Pionki) -> (i8, i8, i8, i8)
{
    let moves = move_generator(p1, p2);

    let no_ve_elems = moves.len();
    let idx = rand::random::<usize>()%no_ve_elems;

    //println!("Agent 1, {}, {}, {}, {}", moves[idx].0, moves[idx].1, moves[idx].2, moves[idx].3);

    return moves[idx];
}

fn agent_1(p1 : Pionki, p2 : Pionki) -> (i8, i8, i8, i8)
{
    let moves = move_generator(p1, p2);
    let mut best_move = moves[0];
    let mut best_res = -1;

    for m in moves
    {
        let mut np1 = p1;
        let mut np2 = p2;

        (np1, np2) = do_move(np1,np2, m);

        let res = rate_situation(np1, np2, 4);
        if res > best_res
        {
            best_res = res;
            best_move = m;
        }
   }
   
    // println!("Agent1, {}, {}, {}, {}", best_move.0, best_move.1, best_move.2, best_move.3);
    return best_move;
}

fn rate_situation(np1: Pionki, np2: Pionki, arg: i32) -> i32 
{
    let mut res = 0;
    for _ in 0..arg
    {
        let mut p1 = np1;
        let mut p2 = np2;

        let mut player = 1;

        let max_steps = 10000;
        while max_steps > 0
        {
            if player == 0
            {
                let mv = agent_losowy(p1, p2);
                (p1,p2) = do_move(p1, p2, mv);

                if !p1.is_empty_pt(&p2.cave)
                {
                    res += 1;
                    break;
                }
            }
            else
            {
                let mv = agent_losowy(p2, p1);
                (p2,p1) = do_move(p2, p1, mv);

                if !p2.is_empty_pt(&p1.cave)
                {
                    break;
                }
            }
            player = 1-player;
        }
    }

    return res;
}

fn agent_2(p1 : Pionki, p2 : Pionki) -> (i8, i8, i8, i8)
{
    let moves = move_generator(p1, p2);

    let mut best_move = moves[0];
    let mut best_res = -1000000;

    let no_mo = moves.len();

    for m in moves
    {
        let mut np1 = p1;
        let mut np2 = p2;

        (np1, np2) = do_move(np1,np2, m);

        //let alpha = -10000;
        //let beta = 10000;
        let (res, _, _) = minimax(np2, np1, 30, false, -10000, 10000);

        if res > best_res
        {
            best_res = res;
            best_move = m;
        }
        else if res == best_res
        {
            let can_get = rand::random::<bool>();
            if can_get
            {
                best_res = res;
                best_move = m;
            }

        }
   }
   
// println!("Agent2 no_mo {}, {}, {}, {}, {}", no_mo, best_move.0, best_move.1, best_move.2, best_move.3);
   
   return best_move;
}

fn minimax(p1: Pionki, p2:Pionki, depth:i32, is_maximizing_player : bool, mut alpha : i32,  mut beta:i32) -> (i32, i32, i32)
{
    if depth == 0 || p1.is_empty_pt(&p2.cave) || p2.is_empty_pt(&p1.cave)
    {
        if is_maximizing_player
        {
            return (heuristic(p1, p2), alpha, beta);
        }
        return (heuristic(p2, p1), alpha, beta);
    }

    if is_maximizing_player
    {
        let moves = move_generator(p1, p2);
        for m in moves
        {
            let mut np1 = p1;
            let mut np2 = p2;

            (np1, np2) = do_move(np1,np2, m);
            let value = minimax(np2, np1, depth-1, !is_maximizing_player, alpha, beta);
            alpha = std::cmp::max( alpha, value.0);
            if beta <= alpha
            {
                break;
            }
        }
        return (alpha,alpha,beta);
    }
    else
    {
        let moves = move_generator(p1, p2);
        for m in moves
        {
            let mut np1 = p1;
            let mut np2 = p2;

            (np1, np2) = do_move(np1,np2, m);

            let value = minimax(np2, np1, depth-1, !is_maximizing_player, alpha, beta);
            beta = std::cmp::min( beta, value.0);
            if beta <= alpha
            {
                break;
            }
        }
        return (beta,alpha, beta);
    }
}

fn heuristic(p1 : Pionki, p2 : Pionki) -> i32
{
    let mut res = 0;
    if p1.rat.is_some()
    {
        res += 10;
    }

    if p1.cat.is_some()
    {
        res += 10;
    }

    if p1.dog.is_some()
    {
        res += 15;
    }

    if p1.wolf.is_some()
    {
        res += 20;
    }

    if p1.panther.is_some()
    {
        res += 25;
    }

    if p1.tiger.is_some()
    {
        res += 30;
    }

    if p1.lion.is_some()
    {
        res += 35;
    }

    if p1.elephant.is_some()
    {
        res += 35;
    }

    if p2.rat.is_some()
    {
        res -= 20;
    }

    if p2.cat.is_some()
    {
        res -= 20;
    }

    if p2.dog.is_some()
    {
        res -= 20;
    }

    if p2.wolf.is_some()
    {
        res -= 20;
    }

    if p2.panther.is_some()
    {
        res -= 25;
    }

    if p2.tiger.is_some()
    {
        res -= 30;
    }

    if p2.lion.is_some()
    {
        res -= 35;
    }

    if p2.elephant.is_some()
    {
        res -= 35;
    }

    if !p1.is_empty_pt(&p2.cave)
    {
        res += 900;
    }

    if !p2.is_empty_pt(&p1.cave)
    {
        res -= 900;
    }

    if p1.cave.y == 0
    {
        if !p1.is_empty_pt(&Pt{x: 2, y: 8})
        {
            res += 50;
        }

        if !p1.is_empty_pt(&Pt{x: 4, y: 8})
        {
            res += 50;
        }

        if !p1.is_empty_pt(&Pt{x: 3, y: 7})
        {
            res += 50;
        }
    }
    else 
    {
        if !p1.is_empty_pt(&Pt{x: 2, y: 0})
        {
            res += 50;
        }

        if !p1.is_empty_pt(&Pt{x: 4, y: 0})
        {
            res += 50;
        }

        if !p1.is_empty_pt(&Pt{x: 3, y: 1})
        {
            res += 50;
        }
    }

    if p1.cave.y == 0
    {
        if !p1.is_empty_pt(&Pt{x:1, y:0}) { res += 5;}
        if !p1.is_empty_pt(&Pt{x:5, y:0}) { res += 5;}
        if !p1.is_empty_pt(&Pt{x:2, y:1}) { res += 8;}
        if !p1.is_empty_pt(&Pt{x:4, y:1}) { res += 8;}
        if !p1.is_empty_pt(&Pt{x:3, y:2}) { res += 9;}
    }
    else 
    {
        if !p1.is_empty_pt(&Pt{x:1, y:8}) { res += 5;}
        if !p1.is_empty_pt(&Pt{x:5, y:8}) { res += 5;}
        if !p1.is_empty_pt(&Pt{x:2, y:7}) { res += 8;}
        if !p1.is_empty_pt(&Pt{x:4, y:7}) { res += 8;}
        if !p1.is_empty_pt(&Pt{x:3, y:6}) { res += 9;}
    }
    
    return res;


}

//0123456
//..#*#..0
//...#...1
//.......2
//.~~.~~.3
//.~~.~~.4
//.~~.~~.5
//.......6
//...#...7
//..#*#..8


fn draw(p1 : Pionki, p2 : Pionki)
{
    let mut board: Vec<Vec<_>> = Vec::new();
    board.push("..#*#..".chars().collect::<Vec<_>>());
    board.push("...#...".chars().collect::<Vec<_>>());
    board.push(".......".chars().collect::<Vec<_>>());
    board.push(".~~.~~.".chars().collect::<Vec<_>>());
    board.push(".~~.~~.".chars().collect::<Vec<_>>());
    board.push(".~~.~~.".chars().collect::<Vec<_>>());
    board.push(".......".chars().collect::<Vec<_>>());
    board.push("...#...".chars().collect::<Vec<_>>());
    board.push("..#*#..".chars().collect::<Vec<_>>());

    if p1.rat.is_some()
    {
        let p = p1.rat.unwrap();
        board[p.y as usize][p.x as usize] = 'r';
    }

    if p1.cat.is_some()
    {
        let p = p1.cat.unwrap();
        board[p.y as usize][p.x as usize] = 'c';
    }

    if p1.dog.is_some()
    {
        let p = p1.dog.unwrap();
        board[p.y as usize][p.x as usize] = 'd';
    }

    if p1.wolf.is_some()
    {
        let p = p1.wolf.unwrap();
        board[p.y as usize][p.x as usize] = 'w';
    }

    if p1.panther.is_some()
    {
        let p = p1.panther.unwrap();
        board[p.y as usize][p.x as usize] = 'j';
    }

    if p1.tiger.is_some()
    {
        let p = p1.tiger.unwrap();
        board[p.y as usize][p.x as usize] = 't';
    }

    if p1.lion.is_some()
    {
        let p = p1.lion.unwrap();
        board[p.y as usize][p.x as usize] = 'l';
    }

    if p1.elephant.is_some()
    {
        let p = p1.elephant.unwrap();
        board[p.y as usize][p.x as usize] = 'e';
    }

    if p2.rat.is_some()
    {
        let p = p2.rat.unwrap();
        board[p.y as usize][p.x as usize] = 'R';
    }

    if p2.cat.is_some()
    {
        let p = p2.cat.unwrap();
        board[p.y as usize][p.x as usize] = 'C';
    }

    if p2.dog.is_some()
    {
        let p = p2.dog.unwrap();
        board[p.y as usize][p.x as usize] = 'D';
    }

    if p2.wolf.is_some()
    {
        let p = p2.wolf.unwrap();
        board[p.y as usize][p.x as usize] = 'W';
    }

    if p2.panther.is_some()
    {
        let p = p2.panther.unwrap();
        board[p.y as usize][p.x as usize] = 'J';
    }

    if p2.tiger.is_some()
    {
        let p = p2.tiger.unwrap();
        board[p.y as usize][p.x as usize] = 'T';
    }

    if p2.lion.is_some()
    {
        let p = p2.lion.unwrap();
        board[p.y as usize][p.x as usize] = 'L';
    }

    if p2.elephant.is_some()
    {
        let p = p2.elephant.unwrap();
        board[p.y as usize][p.x as usize] = 'E';
    }

    let napis = board.into_iter().map(|f| f.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n");
    print!("{}", napis);
    println!();
    println!();


}

fn game() -> bool
{
    let mut p1 : Pionki = Pionki{ rat: Some(Pt{x:6, y:6}), 
                        cat: Some(Pt{x: 1, y: 7}), 
                        dog: Some(Pt{x: 5, y: 7}), 
                        wolf: Some(Pt{x: 2, y: 6}), 
                        panther: Some(Pt{x: 4, y: 6}),
                        tiger: Some(Pt{x: 0, y: 8}),
                        lion: Some(Pt{x: 6, y: 8}),
                        elephant: Some(Pt{x: 0, y: 6}),
                        cave: Pt{x:3, y:8}};

    
    let mut p2 : Pionki = Pionki{ rat: Some(Pt{x:0, y:2}), 
                        cat: Some(Pt{x: 5, y: 1}), 
                        dog: Some(Pt{x: 1, y: 1}), 
                        wolf: Some(Pt{x: 4, y: 2}), 
                        panther: Some(Pt{x: 2, y: 2}),
                        tiger: Some(Pt{x: 6, y: 0}),
                        lion: Some(Pt{x: 0, y: 0}),
                        elephant: Some(Pt{x: 6, y: 2}),
                        cave: Pt{x:3, y:0}};

    let mut player = 0;

    let max_steps = 10000;
    while max_steps > 0
    {
        if player == 0
        {
            let mv = agent_1(p1, p2);
            (p1,p2) = do_move(p1, p2, mv);

            if !p1.is_empty_pt(&p2.cave)
            {
                //println!("Player 1 wins");
                //draw(p1, p2);
                return true;
            }
        }
        else
        {
            let mv = agent_2(p2, p1);
            (p2,p1) = do_move(p2, p1, mv);

            if !p2.is_empty_pt(&p1.cave)
            {
                //println!("Player 2 wins");
                //draw(p1, p2);
                return false;
            }
        }
        player = 1-player;

        //draw(p1, p2);
    }

    return true;

}



fn main() {
    let mut res1 = 0;
    let mut res2 = 0;
    for _ in 0..10
    {
        if game()
        {
            res1 += 1;
        }
        else {
            res2 += 1;
        }
        println!("Player 1 wins: {}, Player 2 wins: {}", res1, res2 );
    }

    // println!("Player 1 wins: {}, Player 2 wins: {}", res1, res2 );
}