#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::policy::{AtomicConstraint, Constraint, Operator, Policy, PolicyKind};

    #[test]
    fn should_deserialize_odrl() {
        let json = json!({
            "@type": "Set",
            "assigner": "assigner",
            "assignee": "assignee",
            "target": "target",
            "obligation": [{
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
               }]
            }],
            "permission": [{
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
               }]
            }],
            "prohibition": [{
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
               }]
            }]
        });

        let policy = serde_json::from_value::<Policy>(json.clone()).unwrap();

        let serialized = serde_json::to_value(&policy).unwrap();

        assert_eq!(&json, &serialized);
    }

    #[test]
    fn should_deserialize_odrl_with_multiplicity_constraints() {
        let json = json!({
            "@type":"Set",
            "assigner":"assigner",
            "assignee":"assignee",
            "target":"target",
            "obligation":[
                {
                    "action":"display",
                    "constraint":[{
                        "and":[
                            {
                                "leftOperand":"spatial",
                                "operator":"eq",
                                "rightOperand":"https://www.wikidata.org/resource/Q183"
                            }
                        ]
                    }]
                }
            ],
        });

        let policy = serde_json::from_value::<Policy>(json.clone()).unwrap();

        let serialized = serde_json::to_value(&policy).unwrap();

        assert_eq!(&json, &serialized);
    }

    #[test]
    fn should_deserialize_edc_prefixed() {
        let json = json!({
            "@id": "b3e9255b-14c9-4a2b-a439-50b9382b81b1",
            "@type": "odrl:Set",
            "odrl:permission": {
                "odrl:action": {
                    "@id": "http://www.w3.org/ns/odrl/2/use"
                },
                "odrl:constraint": {
                    "odrl:leftOperand": {
                      "@id": "https://w3id.org/edc/v0.0.1/ns/foo"
                    },
                    "odrl:operator": {
                        "@id": "odrl:eq"
                    },
                    "odrl:rightOperand": "bar"
                }
            },
            "odrl:prohibition": [],
            "odrl:obligation": []
        });

        let policy = serde_json::from_value::<Policy>(json).unwrap();

        assert_eq!(policy.kind(), &PolicyKind::Set);
        assert_eq!(policy.permissions().len(), 1);

        let permission = &policy.permissions[0];

        assert_eq!(permission.action().id(), "http://www.w3.org/ns/odrl/2/use");
        assert_eq!(permission.constraints().len(), 1);

        let constraint = &permission.constraints()[0];

        assert_eq!(
            constraint,
            &Constraint::Atomic(AtomicConstraint::new_with_operator(
                "https://w3id.org/edc/v0.0.1/ns/foo",
                Operator::id("odrl:eq"),
                "bar"
            ))
        );
    }

    #[test]
    fn should_deserialize_odrl_absolute_path() {
        let content = json!([
          {
            "@id": "cG9zdHMtZnJvbS1saTI=:bGkyLWFzc2V0LTE=:MjRmOTI3MjMtYzZlZS00NGNmLWI0ZWItYTgxYTg3NzJjNGE3",
            "@type": "http://www.w3.org/ns/odrl/2/Offer",
            "http://www.w3.org/ns/odrl/2/permission": [],
            "http://www.w3.org/ns/odrl/2/prohibition": [],
            "http://www.w3.org/ns/odrl/2/obligation": []
          },
          {
            "@id": "dGFtaXMtbWVtYmVyLW9ubHk=:bGkyLWFzc2V0LTE=:OWUyZGE4NGYtMjYwZC00MzFiLWFiN2YtMGQ1YzY0ZTg2Nzcz",
            "@type": "http://www.w3.org/ns/odrl/2/Offer",
            "http://www.w3.org/ns/odrl/2/permission": {
              "http://www.w3.org/ns/odrl/2/action": {
                "@id": "http://www.w3.org/ns/odrl/2/use"
              },
              "http://www.w3.org/ns/odrl/2/constraint": {
                "http://www.w3.org/ns/odrl/2/leftOperand": {
                  "@id": "MembershipCredential"
                },
                "http://www.w3.org/ns/odrl/2/operator": {
                  "@id": "http://www.w3.org/ns/odrl/2/eq"
                },
                "http://www.w3.org/ns/odrl/2/rightOperand": "active"
              }
            },
            "http://www.w3.org/ns/odrl/2/prohibition": [],
            "http://www.w3.org/ns/odrl/2/obligation": []
          },
          {
            "@id": "cG9zdHMtZnJvbS1saTItdjI=:bGkyLWFzc2V0LTE=:OTNkMTMyNmMtZmE5Yy00ZGFlLTk0NTAtMWZiN2RmZTg3NjAz",
            "@type": "http://www.w3.org/ns/odrl/2/Offer",
            "http://www.w3.org/ns/odrl/2/permission": [],
            "http://www.w3.org/ns/odrl/2/prohibition": [],
            "http://www.w3.org/ns/odrl/2/obligation": []
          },
          {
            "@id": "T2ZmZXIxMjM=:bGkyLWFzc2V0LTE=:MTA1ZTk0MjUtN2Y1ZC00N2I3LTlmNzAtNjcxNzJiYTlkMDZk",
            "@type": "http://www.w3.org/ns/odrl/2/Offer",
            "http://www.w3.org/ns/odrl/2/permission": [],
            "http://www.w3.org/ns/odrl/2/prohibition": [],
            "http://www.w3.org/ns/odrl/2/obligation": []
          }
        ]);

        let policy = serde_json::from_value::<Vec<Policy>>(content).unwrap();
        // println!("{:#?}", policy);
        assert_eq!(policy.len(), 4);
    }
}
