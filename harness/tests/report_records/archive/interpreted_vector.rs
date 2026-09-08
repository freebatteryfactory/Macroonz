//! Independently composed complete interpreted records with movable nested claims.

use super::binding_vector::{name, revision};
use super::candidate_vector::Name;
use super::mutation_vector::MutationVector;
use super::parity_vector::{ParityVector, TrialVector, finding};
use super::projection_vector::ProjectionVector;
use super::proposal_vector::TargetVector;
use super::suite_vector::SuiteVector;
use super::surface_vector::{AlternativeVector, PointVector, SurfaceVector};
use super::vector::{frame, hash};

pub(super) struct InterpretedVector {
    pub(super) header: [u32; 3],
    pub(super) surface: SurfaceVector,
    pub(super) suite: SuiteVector,
    pub(super) projection: ProjectionVector,
    pub(super) meaning: Vec<u8>,
    pub(super) report: TrialVector,
    pub(super) mutation: MutationVector,
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = hash("historical-interpreted-evidence/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn selection(surface: &[u8; 32], point: Name, alternative: &[u8; 32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    frame(surface, &mut bytes);
    name(point, &mut bytes);
    frame(alternative, &mut bytes);
    bytes
}

pub(super) fn pair(family: Name, surface: &[u8; 32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    name(family, &mut bytes);
    frame(&revision(&[5; 32], 1), &mut bytes);
    frame(&revision(&[6; 32], 2), &mut bytes);
    frame(surface, &mut bytes);
    bytes
}

pub(super) fn site(label: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    frame(b"outside", &mut bytes);
    frame(b"subject.rs", &mut bytes);
    bytes.extend_from_slice(&11u32.to_be_bytes());
    frame(label, &mut bytes);
    bytes
}

impl InterpretedVector {
    pub(super) fn declared() -> Result<Self, ()> {
        let mut projection = ProjectionVector::declared();
        let [owner, _suite, _subject, _check, _population] = projection.parity.witness.row.names;
        let mut surface = SurfaceVector::declared();
        surface.family = (b"evaluation", b"family");
        surface.points = vec![PointVector {
            name: (b"mutation-owner", b"point"),
            owner,
            original: vec![255],
            site: (b"activation-owner", b"site"),
            alternatives: vec![AlternativeVector {
                family: b"historical-family-outside-current-bank".to_vec(),
                operation: vec![0],
                identity: None,
            }],
            count: None,
        }];
        let address = hash("evaluation-surface/v1", &surface.canonical());
        let point = surface.points.first().ok_or(())?;
        let alternative = point.alternatives.first().ok_or(())?.address(point.name);
        projection.parity.pair = pair(surface.family, &address);
        projection.selection = selection(&address, point.name, &alternative);
        projection.baseline_report.site = site(b"compiled");
        projection.selected_report.site = site(b"compiled");
        let mut report = ParityVector::declared().production_report;
        report.site = site(b"active");
        report.attempt = finding(&projection.parity.witness.trial());
        let mut mutation = MutationVector::demonstrated();
        mutation.equivalence = 0;
        mutation.outcome = vec![0, 0];
        mutation
            .outcome
            .extend(report.attempt.iter().copied().skip(1));
        let mut vector = Self {
            header: [1, 1, 0],
            surface,
            suite: SuiteVector::mixed(),
            projection,
            meaning: vec![0, 255, 13],
            report,
            mutation,
        };
        vector.projection.mutation.target = vector.target(2)?.body();
        vector.mutation.target = vector.target(1)?.body();
        vector.mutation.activation = vector.activation(7)?;
        Ok(vector)
    }

    pub(super) fn target(&self, road: u8) -> Result<TargetVector, ()> {
        let point = self.surface.points.first().ok_or(())?;
        let alternative = point.alternatives.first().ok_or(())?;
        let mut target = TargetVector::selected(road);
        target.identity = vec![road];
        name(point.name, &mut target.identity);
        frame(&alternative.address(point.name), &mut target.identity);
        target.family = vec![1];
        frame(&alternative.family, &mut target.family);
        target.site = vec![1];
        name(point.site, &mut target.site);
        target.owner = vec![1];
        name(point.owner, &mut target.owner);
        Ok(target)
    }

    pub(super) fn activation(&self, firings: u32) -> Result<Vec<u8>, ()> {
        let point = self.surface.points.first().ok_or(())?;
        let alternative = point.alternatives.first().ok_or(())?;
        let address = hash("evaluation-surface/v1", &self.surface.canonical());
        let mut bytes = vec![0];
        bytes.extend(selection(
            &address,
            point.name,
            &alternative.address(point.name),
        ));
        frame(&self.projection.parity.witness.trial(), &mut bytes);
        bytes.extend_from_slice(&firings.to_be_bytes());
        Ok(bytes)
    }

    pub(super) fn members(&self) -> [Vec<u8>; 6] {
        [
            self.surface.encoded(),
            self.suite.encoded(),
            self.projection.encoded(),
            self.meaning.clone(),
            self.report.encoded(),
            self.mutation.encoded(),
        ]
    }

    pub(super) fn body_with(&self, members: &[Vec<u8>; 6]) -> Vec<u8> {
        let mut body = Vec::new();
        for word in self.header {
            body.extend_from_slice(&word.to_be_bytes());
        }
        for member in members {
            frame(member, &mut body);
        }
        body
    }

    pub(super) fn body(&self) -> Vec<u8> {
        self.body_with(&self.members())
    }

    pub(super) fn encoded(&self) -> Vec<u8> {
        envelope(&self.body())
    }
}
