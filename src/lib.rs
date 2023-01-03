/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::ffi::CStr;
use spinners::Spinner;
use spinners::Spinners;

#[repr(C)]
struct spinner_t {
    spinner: *mut c_void
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
enum spinner_spinners_t {
    SPINNER_SPINNERS_DOTS,
    SPINNER_SPINNERS_DOTS2,
    SPINNER_SPINNERS_DOTS3,
    SPINNER_SPINNERS_DOTS4,
    SPINNER_SPINNERS_DOTS5,
    SPINNER_SPINNERS_DOTS6,
    SPINNER_SPINNERS_DOTS7,
    SPINNER_SPINNERS_DOTS8,
    SPINNER_SPINNERS_DOTS9,
    SPINNER_SPINNERS_DOTS10,
    SPINNER_SPINNERS_DOTS11,
    SPINNER_SPINNERS_DOTS12,
    SPINNER_SPINNERS_DOTS8BIT,
    SPINNER_SPINNERS_LINE,
    SPINNER_SPINNERS_LINE2,
    SPINNER_SPINNERS_PIPE,
    SPINNER_SPINNERS_SIMPLEDOTS,
    SPINNER_SPINNERS_SIMPLEDOTSSCROLLING,
    SPINNER_SPINNERS_STAR,
    SPINNER_SPINNERS_STAR2,
    SPINNER_SPINNERS_FLIP,
    SPINNER_SPINNERS_HAMBURGER,
    SPINNER_SPINNERS_GROWVERTICAL,
    SPINNER_SPINNERS_GROWHORIZONTAL,
    SPINNER_SPINNERS_BALLOON,
    SPINNER_SPINNERS_BALLOON2,
    SPINNER_SPINNERS_NOISE,
    SPINNER_SPINNERS_BOUNCE,
    SPINNER_SPINNERS_BOXBOUNCE,
    SPINNER_SPINNERS_BOXBOUNCE2,
    SPINNER_SPINNERS_TRIANGLE,
    SPINNER_SPINNERS_ARC,
    SPINNER_SPINNERS_CIRCLE,
    SPINNER_SPINNERS_SQUARECORNERS,
    SPINNER_SPINNERS_CIRCLEQUARTERS,
    SPINNER_SPINNERS_CIRCLEHALVES,
    SPINNER_SPINNERS_SQUISH,
    SPINNER_SPINNERS_TOGGLE,
    SPINNER_SPINNERS_TOGGLE2,
    SPINNER_SPINNERS_TOGGLE3,
    SPINNER_SPINNERS_TOGGLE4,
    SPINNER_SPINNERS_TOGGLE5,
    SPINNER_SPINNERS_TOGGLE6,
    SPINNER_SPINNERS_TOGGLE7,
    SPINNER_SPINNERS_TOGGLE8,
    SPINNER_SPINNERS_TOGGLE9,
    SPINNER_SPINNERS_TOGGLE10,
    SPINNER_SPINNERS_TOGGLE11,
    SPINNER_SPINNERS_TOGGLE12,
    SPINNER_SPINNERS_TOGGLE13,
    SPINNER_SPINNERS_ARROW,
    SPINNER_SPINNERS_ARROW2,
    SPINNER_SPINNERS_ARROW3,
    SPINNER_SPINNERS_BOUNCINGBAR,
    SPINNER_SPINNERS_BOUNCINGBALL,
    SPINNER_SPINNERS_SMILEY,
    SPINNER_SPINNERS_MONKEY,
    SPINNER_SPINNERS_HEARTS,
    SPINNER_SPINNERS_CLOCK,
    SPINNER_SPINNERS_EARTH,
    SPINNER_SPINNERS_MATERIAL,
    SPINNER_SPINNERS_MOON,
    SPINNER_SPINNERS_RUNNER,
    SPINNER_SPINNERS_PONG,
    SPINNER_SPINNERS_SHARK,
    SPINNER_SPINNERS_DQPB,
    SPINNER_SPINNERS_WEATHER,
    SPINNER_SPINNERS_CHRISTMAS,
    SPINNER_SPINNERS_GRENADE,
    SPINNER_SPINNERS_POINT,
    SPINNER_SPINNERS_LAYER,
    SPINNER_SPINNERS_BETAWAVE,
    SPINNER_SPINNERS_FINGERDANCE,
    SPINNER_SPINNERS_FISTBUMP,
    SPINNER_SPINNERS_SOCCERHEADER,
    SPINNER_SPINNERS_MINDBLOWN,
    SPINNER_SPINNERS_SPEAKER,
    SPINNER_SPINNERS_ORANGEPULSE,
    SPINNER_SPINNERS_BLUEPULSE,
    SPINNER_SPINNERS_ORANGEBLUEPULSE,
    SPINNER_SPINNERS_TIMETRAVEL,
    SPINNER_SPINNERS_AESTHETIC,
}

fn c_spinner_to_rust_spinner(spinner: spinner_spinners_t) -> Spinners {
    match spinner {
	spinner_spinners_t::SPINNER_SPINNERS_DOTS => Spinners::Dots,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS2 => Spinners::Dots2,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS3 => Spinners::Dots3,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS4 => Spinners::Dots4,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS5 => Spinners::Dots5,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS6 => Spinners::Dots6,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS7 => Spinners::Dots7,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS8 => Spinners::Dots8,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS9 => Spinners::Dots9,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS10 => Spinners::Dots10,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS11 => Spinners::Dots11,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS12 => Spinners::Dots12,
	spinner_spinners_t::SPINNER_SPINNERS_DOTS8BIT => Spinners::Dots8Bit,
	spinner_spinners_t::SPINNER_SPINNERS_LINE => Spinners::Line,
	spinner_spinners_t::SPINNER_SPINNERS_LINE2 => Spinners::Line2,
	spinner_spinners_t::SPINNER_SPINNERS_PIPE => Spinners::Pipe,
	spinner_spinners_t::SPINNER_SPINNERS_SIMPLEDOTS => Spinners::SimpleDots,
	spinner_spinners_t::SPINNER_SPINNERS_SIMPLEDOTSSCROLLING => Spinners::SimpleDotsScrolling,
	spinner_spinners_t::SPINNER_SPINNERS_STAR => Spinners::Star,
	spinner_spinners_t::SPINNER_SPINNERS_STAR2 => Spinners::Star2,
	spinner_spinners_t::SPINNER_SPINNERS_FLIP => Spinners::Flip,
	spinner_spinners_t::SPINNER_SPINNERS_HAMBURGER => Spinners::Hamburger,
	spinner_spinners_t::SPINNER_SPINNERS_GROWVERTICAL => Spinners::GrowVertical,
	spinner_spinners_t::SPINNER_SPINNERS_GROWHORIZONTAL => Spinners::GrowHorizontal,
	spinner_spinners_t::SPINNER_SPINNERS_BALLOON => Spinners::Balloon,
	spinner_spinners_t::SPINNER_SPINNERS_BALLOON2 => Spinners::Balloon2,
	spinner_spinners_t::SPINNER_SPINNERS_NOISE => Spinners::Noise,
	spinner_spinners_t::SPINNER_SPINNERS_BOUNCE => Spinners::Bounce,
	spinner_spinners_t::SPINNER_SPINNERS_BOXBOUNCE => Spinners::BoxBounce,
	spinner_spinners_t::SPINNER_SPINNERS_BOXBOUNCE2 => Spinners::BoxBounce2,
	spinner_spinners_t::SPINNER_SPINNERS_TRIANGLE => Spinners::Triangle,
	spinner_spinners_t::SPINNER_SPINNERS_ARC => Spinners::Arc,
	spinner_spinners_t::SPINNER_SPINNERS_CIRCLE => Spinners::Circle,
	spinner_spinners_t::SPINNER_SPINNERS_SQUARECORNERS => Spinners::SquareCorners,
	spinner_spinners_t::SPINNER_SPINNERS_CIRCLEQUARTERS => Spinners::CircleQuarters,
	spinner_spinners_t::SPINNER_SPINNERS_CIRCLEHALVES => Spinners::CircleHalves,
	spinner_spinners_t::SPINNER_SPINNERS_SQUISH => Spinners::Squish,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE => Spinners::Toggle,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE2 => Spinners::Toggle2,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE3 => Spinners::Toggle3,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE4 => Spinners::Toggle4,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE5 => Spinners::Toggle5,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE6 => Spinners::Toggle6,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE7 => Spinners::Toggle7,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE8 => Spinners::Toggle8,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE9 => Spinners::Toggle9,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE10 => Spinners::Toggle10,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE11 => Spinners::Toggle11,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE12 => Spinners::Toggle12,
	spinner_spinners_t::SPINNER_SPINNERS_TOGGLE13 => Spinners::Toggle13,
	spinner_spinners_t::SPINNER_SPINNERS_ARROW => Spinners::Arrow,
	spinner_spinners_t::SPINNER_SPINNERS_ARROW2 => Spinners::Arrow2,
	spinner_spinners_t::SPINNER_SPINNERS_ARROW3 => Spinners::Arrow3,
	spinner_spinners_t::SPINNER_SPINNERS_BOUNCINGBAR => Spinners::BouncingBar,
	spinner_spinners_t::SPINNER_SPINNERS_BOUNCINGBALL => Spinners::BouncingBall,
	spinner_spinners_t::SPINNER_SPINNERS_SMILEY => Spinners::Smiley,
	spinner_spinners_t::SPINNER_SPINNERS_MONKEY => Spinners::Monkey,
	spinner_spinners_t::SPINNER_SPINNERS_HEARTS => Spinners::Hearts,
	spinner_spinners_t::SPINNER_SPINNERS_CLOCK => Spinners::Clock,
	spinner_spinners_t::SPINNER_SPINNERS_EARTH => Spinners::Earth,
	spinner_spinners_t::SPINNER_SPINNERS_MATERIAL => Spinners::Material,
	spinner_spinners_t::SPINNER_SPINNERS_MOON => Spinners::Moon,
	spinner_spinners_t::SPINNER_SPINNERS_RUNNER => Spinners::Runner,
	spinner_spinners_t::SPINNER_SPINNERS_PONG => Spinners::Pong,
	spinner_spinners_t::SPINNER_SPINNERS_SHARK => Spinners::Shark,
	spinner_spinners_t::SPINNER_SPINNERS_DQPB => Spinners::Dqpb,
	spinner_spinners_t::SPINNER_SPINNERS_WEATHER => Spinners::Weather,
	spinner_spinners_t::SPINNER_SPINNERS_CHRISTMAS => Spinners::Christmas,
	spinner_spinners_t::SPINNER_SPINNERS_GRENADE => Spinners::Grenade,
	spinner_spinners_t::SPINNER_SPINNERS_POINT => Spinners::Point,
	spinner_spinners_t::SPINNER_SPINNERS_LAYER => Spinners::Layer,
	spinner_spinners_t::SPINNER_SPINNERS_BETAWAVE => Spinners::BetaWave,
	spinner_spinners_t::SPINNER_SPINNERS_FINGERDANCE => Spinners::FingerDance,
	spinner_spinners_t::SPINNER_SPINNERS_FISTBUMP => Spinners::FistBump,
	spinner_spinners_t::SPINNER_SPINNERS_SOCCERHEADER => Spinners::SoccerHeader,
	spinner_spinners_t::SPINNER_SPINNERS_MINDBLOWN => Spinners::Mindblown,
	spinner_spinners_t::SPINNER_SPINNERS_SPEAKER => Spinners::Speaker,
	spinner_spinners_t::SPINNER_SPINNERS_ORANGEPULSE => Spinners::OrangePulse,
	spinner_spinners_t::SPINNER_SPINNERS_BLUEPULSE => Spinners::BluePulse,
	spinner_spinners_t::SPINNER_SPINNERS_ORANGEBLUEPULSE => Spinners::OrangeBluePulse,
	spinner_spinners_t::SPINNER_SPINNERS_TIMETRAVEL => Spinners::TimeTravel,
	spinner_spinners_t::SPINNER_SPINNERS_AESTHETIC => Spinners::Aesthetic,
    }
}

#[no_mangle]
unsafe extern "C" fn spinner_new(spinner: spinner_spinners_t,
				 message: *const c_char) -> spinner_t {
    let spinner_rs = c_spinner_to_rust_spinner(spinner);
    let message_rs = match CStr::from_ptr(message).to_str() {
	Ok(str) => str,
	Err(_) => ""
    };
    let spinner_result = Spinner::new(spinner_rs, message_rs.to_string());
    spinner_t {
	spinner: Box::into_raw(Box::new(spinner_result)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn spinner_new_with_timer(spinner: spinner_spinners_t,
				 message: *const c_char) -> spinner_t {
    let spinner_rs = c_spinner_to_rust_spinner(spinner);
    let message_rs = match CStr::from_ptr(message).to_str() {
	Ok(str) => str,
	Err(_) => ""
    };
    let spinner_result = Spinner::with_timer(spinner_rs, message_rs.to_string());
    spinner_t {
	spinner: Box::into_raw(Box::new(spinner_result)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn spinner_stop(spinner: *mut spinner_t) {
    let spin = &mut *((*spinner).spinner as *mut Spinner);
    spin.stop();
}

#[no_mangle]
unsafe extern "C" fn spinner_stop_with_symbol(spinner: *mut spinner_t, symbol: *const c_char) {
    let spin = &mut *((*spinner).spinner as *mut Spinner);
    let symbol_rs = match CStr::from_ptr(symbol).to_str() {
	Ok(str) => str,
	Err(_) => ""
    };
    spin.stop_with_symbol(symbol_rs);
}

#[no_mangle]
unsafe extern "C" fn spinner_stop_with_newline(spinner: *mut spinner_t) {
    let spin = &mut *((*spinner).spinner as *mut Spinner);
    spin.stop_with_newline();
}

#[no_mangle]
unsafe extern "C" fn spinner_stop_with_message(spinner: *mut spinner_t, msg: *const c_char) {
    let spin = &mut *((*spinner).spinner as *mut Spinner);
    let msg_rs = match CStr::from_ptr(msg).to_str() {
	Ok(str) => str,
	Err(_) => ""
    };
    spin.stop_with_message(msg_rs.to_string());
}

#[no_mangle]
unsafe extern "C" fn spinner_stop_and_persist(spinner: *mut spinner_t, symbol: *const c_char, msg: *const c_char) {
    let spin = &mut *((*spinner).spinner as *mut Spinner);
    let symbol_rs = match CStr::from_ptr(symbol).to_str() {
	Ok(str) => str,
	Err(_) => ""
    };
    let msg_rs = match CStr::from_ptr(msg).to_str() {
	Ok(str) => str,
	Err(_) => ""
    };
    spin.stop_and_persist(symbol_rs, msg_rs.to_string());
}

#[no_mangle]
unsafe extern "C" fn spinner_clean(spinner: *mut spinner_t) {
    if !spinner.is_null() {
	let _ = Box::from_raw((*spinner).spinner as *mut Spinner);
    }
}

mod test {

    use std::thread::sleep;
    use std::time::Duration;
    use crate::*;
    
    #[test]
    fn test() {
	unsafe {
	    let mut sp = spinner_new(spinner_spinners_t::SPINNER_SPINNERS_DOTS9, "Waiting for 3 seconds\0".as_ptr() as *const c_char);
	    sleep(Duration::from_secs(3));
	    spinner_stop(&mut sp);
	    spinner_clean(&mut sp);
	    
	}
    }
}
