pub mod tguy {
    extern crate unicode_segmentation;

    use unicode_segmentation::UnicodeSegmentation;
    use std::io::Write;
    use std::io::stdout;
    pub struct TrashGuyState<'a> {
        initial_frames_count: u32,
        sprite_right: &'a str,
        sprite_left: &'a str,

        sprite_can: &'a str,

        sprite_space: &'a str,
        text: Vec<&'a str>,

        field: Vec<&'a str>,

        cur_frame: u32,

        max_frames: u32,

        bufsize: usize,
    }

    impl<'a> TrashGuyState<'_> {
        pub fn from_arr_ex(arr: Vec<&'a str>, spacing: u32, sprite_right: &'a str, sprite_left: &'a str, sprite_can: &'a str, sprite_space: &'a str) -> TrashGuyState<'a> {
            let arrlen = arr.len();
            let mut var = TrashGuyState {
                initial_frames_count: (spacing + 1) * 2,
                sprite_right: sprite_right,
                sprite_left: sprite_left,
                sprite_can: sprite_can,
                sprite_space: sprite_space,
                text: arr,
                field: vec![" "; 2 + spacing as usize + arrlen],
                cur_frame: u32::MAX,
                max_frames: 0,
                bufsize: 0,
            };
            var.max_frames = TrashGuyState::get_frame_lower_boundary(var.initial_frames_count,
                                                                     arrlen as u32) + 1;
            var.field[0] = var.sprite_can;
            return var;
        }
        pub fn from_arr(arr: Vec<&str>, spacing: u32) -> TrashGuyState {
            return TrashGuyState::from_arr_ex(arr, spacing, "(> ^_^)>", "<(^_^ <)", "🗑", " ");
        }
        pub fn from_utf8(string: &str, spacing: u32) -> TrashGuyState {
            let mut v: Vec<&str> = vec![];
            for graph in string.graphemes(true).into_iter() {
                v.push(graph);
            }
            return TrashGuyState::from_arr(v, spacing);
        }
        pub fn fprint(&self, fp: &mut dyn Write) {
            for it in self.field.iter() {
                fp.write(it.as_bytes()).expect("RIP");
            }
        }
        pub fn print(&self) {
            self.fprint(&mut stdout());
        }
        pub fn bprint(&self, buf: &mut Vec<u8>) {
            for it in self.field.iter() {
                buf.extend(it.as_bytes());
            }
        }
        pub fn set_frame(&mut self, frame: u32) {
            if self.cur_frame == frame { return; }
            let b = self.initial_frames_count - 1;
            let c = frame;
            let element_index = (((((b * b) + (c << 2)) as f64).sqrt() as u32) - b) / 2;
            let frames_per_element = self.initial_frames_count + (2 * element_index);
            let sub_frame = frame - TrashGuyState::get_frame_lower_boundary(self.initial_frames_count, element_index);
            let right = sub_frame < (frames_per_element / 2);
            let i = if right { sub_frame } else { frames_per_element - sub_frame - 1 };
            self.cur_frame = frame;
            self.clear_field(element_index + !right as u32);
            self.field[(i + 1) as usize] = if right { self.sprite_right } else { self.sprite_left };
            if !right && i != 0 {
                self.field[i as usize] = self.text[element_index as usize];
            }
        }
        pub fn get_frames_count(&self) -> u32 { return self.max_frames; }
        pub fn get_arr(&self) -> &Vec<&str> { return &self.field; }

        #[inline]
        fn get_frame_lower_boundary(initial_frames_count: u32, element_index: u32) -> u32 {
            return element_index * (element_index + initial_frames_count - 1);
        }

        #[inline]
        fn clear_field(&mut self, n_erase_elements: u32) {
            let items_offset = self.field.len() - self.text.len() + n_erase_elements as usize;
            for i in 1..items_offset {
                self.field[i as usize] = self.sprite_space;
            }
            for i in 0..(self.text.len() - n_erase_elements as usize) {
                self.field[items_offset + i] = self.text[n_erase_elements as usize + i];
            }
        }
    }

    impl Iterator for &mut TrashGuyState<'_> {
        type Item = String;
        fn next(&mut self) -> Option<String> {
            if self.cur_frame == self.max_frames {
                self.cur_frame -= 1;
                return None;
            }
            if self.cur_frame == u32::MAX {
                self.set_frame(0);
            }
            let res = self.get_arr().join("");
            if self.cur_frame == self.max_frames - 1 {
                self.set_frame(self.cur_frame);
                self.cur_frame += 1;
            } else {
                self.set_frame(self.cur_frame + 1);
            }
            return Some(res);
        }
    }
}


