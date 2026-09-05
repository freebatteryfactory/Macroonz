//! The four established public path constants are compatibility readings of one enforced magnitude.

use macroonz_compiler::codec::{
    CODEC_PATH_SEGMENT_LIMIT, CodecError, CodecIssue, CodecTypePath, PathRooting,
};
use macroonz_compiler::descriptor::{
    DeclarationError as DescriptorDeclarationError, DirectBinding,
    PATH_SEGMENT_LIMIT as DESCRIPTOR_PATH_SEGMENT_LIMIT, Seat,
};
use macroonz_compiler::stamp::{
    PATH_SEGMENT_LIMIT as STAMP_PATH_SEGMENT_LIMIT, SiteRoot, StampError,
};
use macroonz_compiler::support::{
    BoundPath, CrateFacing, DeclarationError as SupportDeclarationError,
    PATH_SEGMENT_LIMIT as SUPPORT_PATH_SEGMENT_LIMIT,
};

fn segments(count: usize) -> Vec<String> {
    (0..count).map(|position| format!("s{position}")).collect()
}

#[test]
fn every_public_path_road_enforces_the_shared_boundary() {
    assert_eq!(CODEC_PATH_SEGMENT_LIMIT, DESCRIPTOR_PATH_SEGMENT_LIMIT);
    assert_eq!(DESCRIPTOR_PATH_SEGMENT_LIMIT, STAMP_PATH_SEGMENT_LIMIT);
    assert_eq!(STAMP_PATH_SEGMENT_LIMIT, SUPPORT_PATH_SEGMENT_LIMIT);

    for count in [
        DESCRIPTOR_PATH_SEGMENT_LIMIT.saturating_sub(1),
        DESCRIPTOR_PATH_SEGMENT_LIMIT,
    ] {
        assert!(CodecTypePath::spelled(PathRooting::InScope, segments(count)).is_ok());
        assert!(DirectBinding::declared(segments(count)).is_ok());
        assert!(SiteRoot::spelled(segments(count)).is_ok());
        assert!(BoundPath::rooted(CrateFacing::Declaring, segments(count)).is_ok());
    }

    let first_over = DESCRIPTOR_PATH_SEGMENT_LIMIT.saturating_add(1);
    assert_eq!(
        CodecTypePath::spelled(PathRooting::InScope, segments(first_over)),
        Err(CodecError::of(CodecIssue::PathSegmentsUnbounded {
            bound: u64::try_from(CODEC_PATH_SEGMENT_LIMIT).unwrap_or(u64::MAX),
            observed: u64::try_from(first_over).unwrap_or(u64::MAX),
        }))
    );
    assert_eq!(
        DirectBinding::declared(segments(first_over)),
        Err(DescriptorDeclarationError::Unbounded {
            seat: Seat::PathSegment,
            bound: u64::try_from(DESCRIPTOR_PATH_SEGMENT_LIMIT).unwrap_or(u64::MAX),
            observed: u64::try_from(first_over).unwrap_or(u64::MAX),
        })
    );
    assert!(matches!(
        SiteRoot::spelled(segments(first_over)),
        Err(StampError::PathUnbounded { overflow })
            if overflow.capacity == STAMP_PATH_SEGMENT_LIMIT && overflow.offered == first_over
    ));
    assert_eq!(
        BoundPath::rooted(CrateFacing::Declaring, segments(first_over)),
        Err(SupportDeclarationError::PathSegmentsUnbounded)
    );
}
