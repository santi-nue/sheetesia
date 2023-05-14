use std::collections::HashSet;

use opencv::core::*;

use super::piano::{NOTE_ASHARP, NOTE_CSHARP, NOTE_DSHARP, NOTE_FSHARP, NOTE_GSHARP};
use std::convert::TryFrom;

pub struct Octave {
	pub notes: Vec<Note>,
}

pub struct Note {
    pub code: u8,

	pub location: Point,
	pub default_color: Vec3b,
    pub is_accidental: bool,
    pressed: bool
}
lazy_static!{
    static ref ACCIDENTAL_NOTES : HashSet<usize> = {
        let m = HashSet::from([NOTE_ASHARP, NOTE_CSHARP, NOTE_DSHARP, NOTE_FSHARP, NOTE_GSHARP]);
        m
    };
}
const NOTE_OFFSET: i32 = 50;
impl Octave {

    pub fn new(octave_location: Point, image: &Mat, template: &Mat) -> Octave {
        let mut notes: Vec<Note> = Vec::new();
        let octave_image = Mat::roi(&image, Rect::from_point_size(octave_location, template.size().unwrap())).unwrap();
        let pixel_diff_thresh = 100;
        // For each note in the octave
        for note in 0usize..12usize {
            
            // Variables to store the bounds of the current note
            let mut min_x = (template.cols()*i32::try_from(note).unwrap())/12;
            let mut max_x = (template.cols()*(i32::try_from(note).unwrap()+1))/12;

            // For each pixel in the note's pixel range
            for x in min_x..max_x {
                // Check for a pixel whose color is close to
                //	the color of the templates' pixel in the same position
                let octave_pixel: Vec3b = *octave_image.at_2d(0, x).unwrap();
                let template_pixel: Vec3b = *template.at_2d(0, x).unwrap();

                // If the color is close (difference < the thresh)
                //	Add note to the notes vector and stop looping
                let mut pixel_diff: i32 = 0;
                for i in 0..3 {
                    pixel_diff += (template_pixel[i] as i32 - octave_pixel[i] as i32).abs();
                }
                if pixel_diff < pixel_diff_thresh {
                    if x < min_x {min_x = x;}
                    if x > max_x {max_x = x;}
                }
            }
            let accidental = ACCIDENTAL_NOTES.contains(&note);
            let avg_x = (min_x+max_x)/2;
            let y_offset = if accidental { -NOTE_OFFSET } else { NOTE_OFFSET };
            notes.push(Note {
                code: note as u8,
                location: octave_location + Point {x: avg_x, y: y_offset}, // Use a point in the middle
                default_color: *octave_image.at_2d(0, avg_x).unwrap(), // Color in that point
                is_accidental: accidental,
                pressed: false
            });
        }

        return Octave {
            notes: notes
        }
    }

}

impl Note {
    // Return Err if the note was in a state and is set to the same state:
    //  If the note was pressed and is set to pressed
    //  If the note wasn't pressed and is set to not pressed
    pub fn set_pressed(&mut self, pressed: bool) -> Result<bool,bool> {
        if self.pressed == pressed {
            return Err(false);
        }

        self.pressed = pressed;
        return Ok(pressed);
    }

    // Return human readable note name
    pub fn to_string(&self) -> String {
        let octave_number = (self.code/12) - 1; 
        let note_number: usize = self.code as usize % 12;

        let note_strings = vec!["C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "];
        return format!("{} {}", note_strings[note_number], octave_number);
    }
}