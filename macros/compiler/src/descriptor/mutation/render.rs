//! The token half of the mutation road: the module a mutation harness lowers and invokes.
//!
//! # What is rendered, and what is only carried
//!
//! The discovery, the policy, and the dispatch are rendered from the declaration.
//! The production expression and every alternative's meaning are carried through unread: they are token material the door computed from the declaration it captured, and a renderer that interpreted them would be deciding what the consumer's declaration means.
//!
//! # Nothing spells a crate
//!
//! Every path begins with the harness binding's own metavariable, so a consumer that renamed the dependency gets its own name back.

use super::{Alternative, Permission, Policy, Site, Surface};
use crate::bounded::Overflow;
use crate::descriptor::vocabulary::{self, HarnessName};
use crate::descriptor::{Name, TypeName};
use crate::stamp::{Visibility, declared_reach_tokens};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, and_all, bound_local, call, comma_many, constant, equality,
    function, group, method_call, method_chain, result_type, roster, text_pair,
};

/// The refusal arm a refused namespaced reference reaches.
const NAME_ARM: &str = "Name";

/// The refusal arm a refused permission reaches.
const PERMISSION_ARM: &str = "Permission";

/// The refusal arm a refused policy reaches.
const POLICY_ARM: &str = "Policy";

/// The refusal arm a refused discovery reaches.
const DISCOVERY_ARM: &str = "Discovery";

/// The refusal arm a refused lowering reaches.
const LOWERING_ARM: &str = "Lowering";

/// The refusal arm a slug the address does not resolve reaches.
const UNRESOLVED_ARM: &str = "OperatorFamilyNotFound";

/// The road a bench target invokes to lower the rendered surface.
const LOWERING: &str = "lowering";

/// The road the unchanged declaration answers through.
const PRODUCTION: &str = "production";

/// The road an evaluation is called through.
const EVALUATION: &str = "evaluation";

/// The road the rendered alternatives are read back through.
const CANDIDATES: &str = "candidate_orders";

/// The local the lowering binds its evaluation family to.
const FAMILY_LOCAL: &str = "family";

/// The local the lowering binds its complete policy to.
const POLICY_LOCAL: &str = "policy";

/// The local the site binds its point to.
const POINT_LOCAL: &str = "point";

/// The local the site binds its activation to.
const ACTIVATION_LOCAL: &str = "activation";

/// The local the site binds its owner claim to.
const OWNER_CLAIM_LOCAL: &str = "owner_claim";

/// The local the lowering binds its one discovered site to.
const SITE_LOCAL: &str = "site";

/// The local an evaluation binds its resolved selection to.
const RESOLVED_LOCAL: &str = "resolved";

/// The parameter every rendered road takes and none of them reads.
const INPUT_PARAMETER: &str = "_input";

/// The lifetime an evaluation directive is borrowed for.
const SURFACE_LIFETIME: &str = "surface";

/// The stem every rendered alternative constant is named from.
const ALTERNATIVE_STEM: &str = "ALTERNATIVE";

/// Render the module a mutation harness lowers and invokes.
///
/// # Errors
///
/// Returns [`Overflow`] where the module outgrows the declared magnitude.
pub fn generated_module(surface: &Surface) -> Result<Vec<GeneratedToken>, Overflow> {
    let refusal = &surface.address().refusal;
    let site = surface.site();
    let mut body = refusal_type(refusal)?;
    body.extend(alternative_constants(site));
    body.extend(candidate_orders(site)?);
    body.extend(lowering(surface)?);
    body.extend(production(site)?);
    body.extend(evaluation(site)?);
    let mut tokens = declared_reach_tokens(Visibility::Crate)?;
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(surface.address().module.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The refusal type the module declares and every lowering road answers in.
fn refusal_type(refusal: &TypeName) -> Result<Vec<GeneratedToken>, Overflow> {
    let carried = [
        (NAME_ARM, HarnessName::Descriptor, HarnessName::NameRefusal),
        (
            PERMISSION_ARM,
            HarnessName::Muterprater,
            HarnessName::PermissionRefusal,
        ),
        (
            POLICY_ARM,
            HarnessName::Muterprater,
            HarnessName::PolicyRefusal,
        ),
        (
            DISCOVERY_ARM,
            HarnessName::Muterprater,
            HarnessName::DiscoveryRefusal,
        ),
        (
            LOWERING_ARM,
            HarnessName::Muterprater,
            HarnessName::DiscoveryLoweringRefusal,
        ),
    ];
    let mut arms: Vec<GeneratedToken> = Vec::new();
    for (arm, module, kind) in carried {
        arms.push(GeneratedToken::word(arm));
        arms.push(group(
            GeneratedDelimiter::Parenthesis,
            vocabulary::path(&[module, kind]),
        )?);
        arms.push(GeneratedToken::alone(','));
    }
    arms.push(GeneratedToken::word(UNRESOLVED_ARM));
    arms.push(GeneratedToken::alone(','));

    let mut tokens = vec![
        GeneratedToken::alone('#'),
        group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("derive"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("Debug")],
                )?,
            ],
        )?,
    ];
    tokens.extend(declared_reach_tokens(Visibility::Crate)?);
    tokens.push(GeneratedToken::word("enum"));
    tokens.push(GeneratedToken::word(refusal.spelling()));
    tokens.push(group(GeneratedDelimiter::Brace, arms)?);
    Ok(tokens)
}

/// The name one alternative's rendered constant carries.
fn alternative_name(position: usize) -> String {
    format!("{ALTERNATIVE_STEM}_{position}")
}

/// One constant per declared alternative, each holding the value that alternative means.
fn alternative_constants(site: &Site) -> Vec<GeneratedToken> {
    let mut tokens: Vec<GeneratedToken> = Vec::new();
    for (position, alternative) in site.alternatives().iter().enumerate() {
        tokens.extend(constant(
            &alternative_name(position),
            site.order().to_vec(),
            alternative.meaning().to_vec(),
        ));
    }
    tokens
}

/// The road the rendered alternatives are read back through, as one array of the declared width.
fn candidate_orders(site: &Site) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut values: Vec<GeneratedToken> = Vec::new();
    for position in 0..site.alternatives().len() {
        values.push(GeneratedToken::word(&alternative_name(position)));
        values.push(GeneratedToken::alone(','));
    }
    let mut result = site.order().to_vec();
    result.push(GeneratedToken::alone(';'));
    result.push(GeneratedToken::number(
        u64::try_from(site.alternatives().len()).unwrap_or(u64::MAX),
    ));
    let mut tokens = declared_reach_tokens(Visibility::Crate)?;
    tokens.extend(function(
        CANDIDATES,
        Vec::new(),
        vec![group(GeneratedDelimiter::Bracket, result)?],
        vec![group(GeneratedDelimiter::Bracket, values)?],
    )?);
    Ok(tokens)
}

/// One expression with `.map_err(<Refusal>::<arm>)?` on it.
fn mapped(
    expression: Vec<GeneratedToken>,
    refusal: &TypeName,
    arm: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = method_call(expression, "map_err", arm_path(refusal, arm))?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One expression with `.ok_or(<Refusal>::<arm>)?` on it.
fn or_refused(
    expression: Vec<GeneratedToken>,
    refusal: &TypeName,
    arm: &str,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = method_call(expression, "ok_or", arm_path(refusal, arm))?;
    tokens.push(GeneratedToken::alone('?'));
    Ok(tokens)
}

/// One arm of the rendered refusal type, as a path.
fn arm_path(refusal: &TypeName, arm: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(refusal.spelling()),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(arm),
    ]
}

/// One namespaced name as the two text literals its parser takes.
fn name_arguments(name: &Name) -> Vec<GeneratedToken> {
    text_pair(name.namespace(), name.stem())
}

/// One byte string as the owned vector the address's constructors take.
fn owned_bytes(material: &[u8]) -> Result<Vec<GeneratedToken>, Overflow> {
    method_chain(vec![GeneratedToken::byte_text(material)], &["to_vec"])
}

/// The locals the policy is built from: the evaluation family, then one claim, its families, and its permission per declared row.
fn policy_locals(policy: &Policy, refusal: &TypeName) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = bound_local(
        FAMILY_LOCAL,
        mapped(
            vocabulary::road(
                &[
                    HarnessName::Muterprater,
                    HarnessName::EvaluationFamilyRef,
                    HarnessName::Named,
                ],
                name_arguments(policy.family()),
            )?,
            refusal,
            NAME_ARM,
        )?,
    );
    for (position, permission) in policy.permissions().iter().enumerate() {
        body.extend(permission_locals(permission, position, refusal)?);
    }
    let mut named: Vec<GeneratedToken> = Vec::new();
    for position in 0..policy.permissions().len() {
        named.push(GeneratedToken::word(&format!("permission_{position}")));
        named.push(GeneratedToken::alone(','));
    }
    body.extend(bound_local(
        POLICY_LOCAL,
        mapped(
            vocabulary::road(
                &[
                    HarnessName::Muterprater,
                    HarnessName::MutationPolicy,
                    HarnessName::Declared,
                ],
                comma_many(vec![
                    vec![GeneratedToken::word(FAMILY_LOCAL)],
                    roster(named)?,
                ]),
            )?,
            refusal,
            POLICY_ARM,
        )?,
    ));
    Ok(body)
}

/// One permission's own locals: its claim, one local per operator family it names, and the permission itself.
fn permission_locals(
    permission: &Permission,
    position: usize,
    refusal: &TypeName,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let claim = format!("permission_claim_{position}");
    let mut body = bound_local(
        &claim,
        mapped(
            vocabulary::road(
                &[
                    HarnessName::Descriptor,
                    HarnessName::ClaimRef,
                    HarnessName::Named,
                ],
                name_arguments(permission.claim()),
            )?,
            refusal,
            NAME_ARM,
        )?,
    );
    let mut named: Vec<GeneratedToken> = Vec::new();
    for (seat, family) in permission.families().iter().enumerate() {
        let local = format!("permission_family_{position}_{seat}");
        body.extend(bound_local(
            &local,
            or_refused(family_reference(family.slug())?, refusal, UNRESOLVED_ARM)?,
        ));
        named.push(GeneratedToken::word(&local));
        named.push(GeneratedToken::alone(','));
    }
    body.extend(bound_local(
        &format!("permission_{position}"),
        mapped(
            vocabulary::road(
                &[
                    HarnessName::Muterprater,
                    HarnessName::MutationPermission,
                    HarnessName::Declared,
                ],
                comma_many(vec![vec![GeneratedToken::word(&claim)], roster(named)?]),
            )?,
            refusal,
            PERMISSION_ARM,
        )?,
    ));
    Ok(body)
}

/// One operator family, resolved from the slug the declaration named it by.
fn family_reference(slug: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    vocabulary::road(
        &[
            HarnessName::Muterprater,
            HarnessName::OperatorFamilyRef,
            HarnessName::OfSlug,
        ],
        vec![GeneratedToken::text(slug)],
    )
}

/// The locals the one discovered site is built from.
fn site_locals(
    site: &Site,
    policy: &Policy,
    refusal: &TypeName,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut body = bound_local(
        POINT_LOCAL,
        mapped(
            vocabulary::road(
                &[
                    HarnessName::Descriptor,
                    HarnessName::MutationPointRef,
                    HarnessName::Named,
                ],
                name_arguments(site.point()),
            )?,
            refusal,
            NAME_ARM,
        )?,
    );
    body.extend(bound_local(
        ACTIVATION_LOCAL,
        mapped(
            vocabulary::road(
                &[
                    HarnessName::Muterprater,
                    HarnessName::ActivationSite,
                    HarnessName::Named,
                ],
                name_arguments(site.point()),
            )?,
            refusal,
            NAME_ARM,
        )?,
    ));
    let mapping = match policy.claim_for(site.fact()) {
        Some(claim) => {
            body.extend(bound_local(
                OWNER_CLAIM_LOCAL,
                mapped(
                    vocabulary::road(
                        &[
                            HarnessName::Descriptor,
                            HarnessName::ClaimRef,
                            HarnessName::Named,
                        ],
                        name_arguments(claim),
                    )?,
                    refusal,
                    NAME_ARM,
                )?,
            ));
            call(
                vocabulary::path(&[
                    HarnessName::Muterprater,
                    HarnessName::OwnerClaimMapping,
                    HarnessName::Mapped,
                ]),
                vec![GeneratedToken::word(OWNER_CLAIM_LOCAL)],
            )?
        }
        None => vocabulary::path(&[
            HarnessName::Muterprater,
            HarnessName::OwnerClaimMapping,
            HarnessName::OwnerUnmapped,
        ]),
    };
    body.extend(bound_local(
        SITE_LOCAL,
        mapped(discovered(site, mapping, refusal)?, refusal, DISCOVERY_ARM)?,
    ));
    Ok(body)
}

/// The one discovered site, over the point, the owner-claim mapping, the unchanged operation, and every declared alternative.
fn discovered(
    site: &Site,
    mapping: Vec<GeneratedToken>,
    refusal: &TypeName,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut candidates: Vec<GeneratedToken> = Vec::new();
    for alternative in site.alternatives() {
        candidates.extend(stated(alternative, refusal)?);
        candidates.push(GeneratedToken::alone(','));
    }
    vocabulary::road(
        &[
            HarnessName::Muterprater,
            HarnessName::DiscoveredMutationSite,
            HarnessName::Discovered,
        ],
        comma_many(vec![
            vec![GeneratedToken::word(POINT_LOCAL)],
            mapping,
            owned_bytes(site.unchanged())?,
            roster(candidates)?,
            vec![GeneratedToken::word(ACTIVATION_LOCAL)],
        ]),
    )
}

/// One declared alternative, as the address's own declaration of it.
fn stated(alternative: &Alternative, refusal: &TypeName) -> Result<Vec<GeneratedToken>, Overflow> {
    let family = or_refused(
        family_reference(alternative.family().slug())?,
        refusal,
        UNRESOLVED_ARM,
    )?;
    vocabulary::road(
        &[
            HarnessName::Muterprater,
            HarnessName::AlternativeDeclaration,
            HarnessName::Stated,
        ],
        comma_many(vec![family, owned_bytes(alternative.operation())?]),
    )
}

/// The road a harness invokes to lower this surface's one discovered site under its policy.
fn lowering(surface: &Surface) -> Result<Vec<GeneratedToken>, Overflow> {
    let refusal = &surface.address().refusal;
    let policy = surface.policy();
    let mut body = policy_locals(policy, refusal)?;
    body.extend(site_locals(surface.site(), policy, refusal)?);
    let lowered = vocabulary::road(
        &[
            HarnessName::Muterprater,
            HarnessName::Discover,
            HarnessName::LowerDiscoveries,
        ],
        comma_many(vec![
            vec![
                GeneratedToken::alone('&'),
                GeneratedToken::word(POLICY_LOCAL),
            ],
            roster(vec![
                GeneratedToken::word(SITE_LOCAL),
                GeneratedToken::alone(','),
            ])?,
        ]),
    )?;
    body.extend(method_call(
        lowered,
        "map_err",
        arm_path(refusal, LOWERING_ARM),
    )?);
    let mut tokens = declared_reach_tokens(Visibility::Crate)?;
    tokens.extend(function(
        LOWERING,
        Vec::new(),
        result_type(
            vocabulary::path(&[
                HarnessName::Muterprater,
                HarnessName::MutationSurfaceLowering,
            ]),
            vec![GeneratedToken::word(refusal.spelling())],
        ),
        body,
    )?);
    Ok(tokens)
}

/// The road the unchanged declaration answers through.
fn production(site: &Site) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = declared_reach_tokens(Visibility::Crate)?;
    tokens.extend(function(
        PRODUCTION,
        input_parameter()?,
        site.order().to_vec(),
        site.production().to_vec(),
    )?);
    Ok(tokens)
}

/// The one parameter every rendered road takes and none of them reads.
fn input_parameter() -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::word(INPUT_PARAMETER),
        GeneratedToken::alone(':'),
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Parenthesis, Vec::new())?,
    ])
}

/// The road an evaluation is called through: the production where nothing was selected, the selected alternative where one was, and a refusal where the selection names something this rendering does not carry.
fn evaluation(site: &Site) -> Result<Vec<GeneratedToken>, Overflow> {
    let observation = observation_type(site);
    let mut body = unselected()?;
    for (position, alternative) in site.alternatives().iter().enumerate() {
        body.push(GeneratedToken::word("if"));
        body.extend(active_condition(site, alternative)?);
        body.push(group(
            GeneratedDelimiter::Brace,
            observed(vec![GeneratedToken::word(&alternative_name(position))], 1)?,
        )?);
    }
    body.extend(call(
        vec![GeneratedToken::word("Err")],
        call(
            vocabulary::path(&[
                HarnessName::Muterprater,
                HarnessName::EvaluationCallRefusal,
                HarnessName::ActiveSelectionNotImplemented,
            ]),
            method_chain(
                vec![GeneratedToken::word(RESOLVED_LOCAL)],
                &[HarnessName::Selection.spelling()],
            )?,
        )?,
    )?);

    let mut tokens = declared_reach_tokens(Visibility::Crate)?;
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word(EVALUATION));
    tokens.extend(surface_lifetime());
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        directive_parameters()?,
    )?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(result_type(
        observation,
        vocabulary::path(&[HarnessName::Muterprater, HarnessName::EvaluationCallRefusal]),
    ));
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The lifetime an evaluation directive is borrowed for, as the tokens that spell it.
fn surface_lifetime() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::alone('<'),
        GeneratedToken::joint('\''),
        GeneratedToken::word(SURFACE_LIFETIME),
        GeneratedToken::alone('>'),
    ]
}

/// The parameters an evaluation takes: the input nothing reads, and the directive it answers under.
fn directive_parameters() -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = input_parameter()?;
    tokens.push(GeneratedToken::alone(','));
    tokens.push(GeneratedToken::word("directive"));
    tokens.push(GeneratedToken::alone(':'));
    tokens.extend(vocabulary::path(&[
        HarnessName::Muterprater,
        HarnessName::EvaluationDirective,
    ]));
    tokens.extend(surface_lifetime());
    Ok(tokens)
}

/// The observation an evaluation answers with, over the type the alternatives are values of.
fn observation_type(site: &Site) -> Vec<GeneratedToken> {
    let mut tokens =
        vocabulary::path(&[HarnessName::Muterprater, HarnessName::EvaluationObservation]);
    tokens.push(GeneratedToken::alone('<'));
    tokens.extend(site.order().to_vec());
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// The `let … else` that answers with the production where the directive resolved no selection.
fn unselected() -> Result<Vec<GeneratedToken>, Overflow> {
    let resolved = method_chain(
        vec![GeneratedToken::word("directive")],
        &[HarnessName::Resolved.spelling()],
    )?;
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word("Some"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word(RESOLVED_LOCAL)],
        )?,
        GeneratedToken::alone('='),
    ];
    tokens.extend(resolved);
    tokens.push(GeneratedToken::word("else"));
    tokens.push(group(
        GeneratedDelimiter::Brace,
        observed(
            call(
                vec![GeneratedToken::word(PRODUCTION)],
                vec![GeneratedToken::word(INPUT_PARAMETER)],
            )?,
            0,
        )?,
    )?);
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

/// One `return Ok(<observation>);` over the meaning an evaluation answers with and how many times it fired.
fn observed(meaning: Vec<GeneratedToken>, firings: u64) -> Result<Vec<GeneratedToken>, Overflow> {
    let taken = vocabulary::road(
        &[
            HarnessName::Muterprater,
            HarnessName::EvaluationObservation,
            HarnessName::Observed,
        ],
        comma_many(vec![meaning, vec![GeneratedToken::number(firings)]]),
    )?;
    let mut tokens = vec![GeneratedToken::word("return")];
    tokens.extend(call(vec![GeneratedToken::word("Ok")], taken)?);
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

/// The condition one alternative's arm fires under: the point, the operator family, and the operation, all compared against what the resolved selection carries.
fn active_condition(
    site: &Site,
    alternative: &Alternative,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let point = site.point();
    let comparisons = vec![
        equality(
            selected_point(&[HarnessName::NamespaceRoad, HarnessName::Written])?,
            vec![GeneratedToken::text(point.namespace())],
        ),
        equality(
            selected_point(&[HarnessName::StemRoad, HarnessName::Written])?,
            vec![GeneratedToken::text(point.stem())],
        ),
        equality(
            method_chain(
                vec![GeneratedToken::word(RESOLVED_LOCAL)],
                &[
                    HarnessName::Alternative.spelling(),
                    HarnessName::Family.spelling(),
                    HarnessName::Slug.spelling(),
                ],
            )?,
            vec![GeneratedToken::text(alternative.family().slug())],
        ),
        equality(
            method_chain(
                vec![GeneratedToken::word(RESOLVED_LOCAL)],
                &[
                    HarnessName::Alternative.spelling(),
                    HarnessName::Operation.spelling(),
                ],
            )?,
            vec![GeneratedToken::byte_text(alternative.operation())],
        ),
    ];
    Ok(and_all(comparisons))
}

/// One reading of the resolved selection's own point name.
fn selected_point(part: &[HarnessName]) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut roads = vec![
        HarnessName::Point.spelling(),
        HarnessName::Identity.spelling(),
        HarnessName::NameRoad.spelling(),
    ];
    roads.extend(part.iter().map(|road| road.spelling()));
    method_chain(vec![GeneratedToken::word(RESOLVED_LOCAL)], &roads)
}
