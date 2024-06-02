use x86_64::structures::tss::TaskStateSegment;

pub static TSS: TaskStateSegment = TaskStateSegment::new();
