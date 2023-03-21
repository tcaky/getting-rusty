//use std::time::{Duration, Instant};
use chrono::{Utc, DateTime, Local, TimeZone
    //Duration as ChronoDuration, 
    //FixedOffset, 
    //Datelike, 
    //Timelike
};
use chrono_tz::Canada::Eastern;
//use chrono_tz::OffsetComponents;

fn main() {
    // let now: Instant = Instant::now();

    // for i in 0..1_000_000_00 {
    //     let _ = i;
    // }

    // let duration: Duration = now.elapsed();

    // dbg!(&duration);
    // dbg!(&duration.as_millis());
    // dbg!(&duration.as_secs());
    
    // Server is set to UTC.  The two lines below should return 
    // the same time (within fractions of a second)
    let now: DateTime<Utc> = Utc::now();
    dbg!(Local::now());
    dbg!(&now);

    // let future: Option<DateTime<Utc>> = now.checked_add_signed(ChronoDuration::weeks(2));
    // dbg!(&future);
    // let past: Option<DateTime<Utc>> = now.checked_sub_signed(ChronoDuration::days(60));
    // dbg!(&past);

    // ***** ONE WAY OF DOING THE OFFSET FOR EASTERN CANADA *****
    // let hour = 3600; // seconds in 1 hour
    // let tz_adjust = FixedOffset::west_opt(4 * hour);
    // dbg!(tz_adjust);
    // let tz_unwrap = tz_adjust.unwrap();
    // dbg!(tz_unwrap);
    // dbg!(now.with_timezone(&tz_unwrap));

    // let (is_pm, hour) = now.hour12();
    // println!("{} {}", hour, if is_pm {"PM"} else {"AM"});
    
    

    // ***** SECOND WAY OF DOING THE OFFSET FOR EASTERN CANADA *****
    let ottawa_time = Eastern.from_utc_datetime(&now.naive_utc());
    dbg!(ottawa_time);
    // ******* THE ABOVE LINE IS THE WINNER! ******
    
    // let yow_utc_offset = ottawa_time.offset().base_utc_offset();
    // let yow_dst_offset = ottawa_time.offset().dst_offset();
    // let total_offset = yow_utc_offset + yow_dst_offset;
    
    // dbg!(&yow_utc_offset);
    // dbg!(&yow_dst_offset);
    // dbg!(&total_offset);
    // let past: Option<DateTime<Utc>> = now.checked_add_signed(total_offset);
    // dbg!(past);
    
}

