//! Band 16 — image: `ProgramImage`, `.tpk`, the packaging profiles, the
//! validation ladder, and the admission pipeline.

pub mod types;

pub use types::{
    ADMISSION_PIPELINE, ADMISSION_PROVES, AdmittedProgramId, AgreementCheckedImage,
    BOUND_FACT_ROSTER, BoundedDecodedImage, ComponentCarriage, ComponentRole, ExecutableImage,
    ImageDigest, ImageFamilyFormatVersion, ImageFamilyId, ImageProfileId, ImageProfileVersion,
    ImageValidation, PROGRAM_IMAGE_EXTENSION, PackagingProfile, ProgramImage,
    ProgramImageComponent, ProgramImageRef, SemanticImage, UntrustedImageBytes,
};
