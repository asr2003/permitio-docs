import React from "react";

export const firstRow = [
  {
    type: "link",
    href: "quickstart",
    label: "Overview",
    svgIcon: <i className="ri-earth-line ri-xl !text-[#974ef2]"></i>,
    description: "Learn what Permit is and how it works",
  },
  {
    type: "link",
    href: "walkthroughs",
    label: "Tutorials & Walkthroughs",
    svgIcon: <i className="ri-book-open-line ri-xl !text-[#974ef2]"></i>,
    description: "Best practices and how-to guides",
  },
  {
    type: "link",
    href: "sdk/sdks-overview",
    label: "SDKs",
    svgIcon: <i className="ri-tools-line ri-xl !text-[#974ef2]"></i>,
    description: "Explore the supported SDKs and frameworks",
  },
  {
    type: "link",
    href: "category/learn-by-example",
    label: "Examples",
    svgIcon: <i className="ri-suitcase-line ri-xl !text-[#974ef2]"></i>,
    description: "View real-life Permit integration examples",
  },
];
export const secondRow = [
  {
    type: "link",
    href: "/concepts/multi-tenant-authorization",
    label: "Multi-Tenancy",
    svgIcon: <i className="ri-hotel-line ri-xl !text-[#974ef2]"></i>,
    description: "Manage multi-tenant access with Permit",
  },
  {
    type: "link",
    href: "/integrations/gitops/overview",
    label: "GitOps",
    svgIcon: <i className="ri-loop-left-line ri-xl !text-[#974ef2]"></i>,
    description: "GitOps-ready Permission Management",
  },
  {
    type: "link",
    href: "/integrations/infra-as-code/terraform-provider",
    label: "Terraform",
    svgIcon: <i className="ri-cloud-line ri-xl !text-[#974ef2]"></i>,
    description: "Manage policy as code with Terraform",
  },
  {
    type: "link",
    href: "/embeddable-uis/overview",
    label: "Permit Elements",
    svgIcon: <i className="ri-palette-line ri-xl !text-[#974ef2]"></i>,
    description: "Pre-built, embeddable, UI components for your app",
  },
  {
    type: "link",
    href: "/integrations/feature-flagging/casl",
    label: "Feature Toggling",
    svgIcon: <i className="ri-brush-line ri-xl !text-[#974ef2]"></i>,
    description: "Adjust & Render UI dynamically based on policy rules",
  },
];
export const policyModelingRow = [
  {
    type: "link",
    href: "/how-to/build-policies/rbac/overview",
    label: "RBAC (Role-Based Access Control)",
    svgIcon: <i className="ri-profile-line ri-xl !text-[#2D7DD2]"></i>,
    description: "Role-based authorization explained",
  },
  {
    type: "link",
    href: "/how-to/build-policies/abac/overview",
    label: "ABAC (Attribute-Based Access Control)",
    svgIcon: <i className="ri-award-line ri-xl !text-[#2D7DD2]"></i>,
    description: "Policy-based access using attributes",
  },
  {
    type: "link",
    href: "/how-to/build-policies/rebac/overview",
    label: "ReBAC (Relationship-Based Access Control)",
    svgIcon: <i className="ri-node-tree ri-xl !text-[#2D7DD2]"></i>,
    description: "Access control based on relationships",
  },
  {
    type: "link",
    href: "/how-to/build-policies/policy-basics",
    label: "Choosing the Right Model",
    svgIcon: <i className="ri-question-line ri-xl !text-[#2D7DD2]"></i>,
    description: "Policy Basics & How to decide between RBAC, ABAC and ReBAC",
  },
  {
    type: "link",
    href: "/how-to/SDLC/modeling-implementation-components",
    label: "Common Policy Patterns",
    svgIcon: <i className="ri-function-line ri-xl !text-[#2D7DD2]"></i>,
    description: "Time-Based, Ownership, Multi-Tenancy",
  },
];
export const enforcementRow = [
  {
    type: "link",
    href: "/overview/sync-your-first-user-with-sdk",
    label: "Syncing Users & Identity",
    svgIcon: <i className="ri-user-settings-line ri-xl !text-[#E67E22]"></i>,
    description: "Sync identities & manage user roles efficiently",
  },
  {
    type: "link",
    href: "/integrations/gateways/overview",
    label: "Gateways & Proxies",
    svgIcon: <i className="ri-shield-line ri-xl !text-[#E67E22]"></i>,
    description: "Enforce policies at the gateway/proxy level",
  },
  {
    type: "link",
    href: "",
    label: "Bulk Operations",
    svgIcon: <i className="ri-stack-line ri-xl !text-[#E67E22]"></i>,
    description: "Efficiently manage permissions in bulk",
  },
  {
    type: "link",
    label: "URL Mapping",
    svgIcon: <i className="ri-link-line ri-xl !text-[#E67E22]"></i>,
    description: "Map and secure routes with access control",
  },
];
export const devOpsRow = [
  {
    type: "link",
    label: "Policy CI/CD",
    svgIcon: <i className="ri-git-branch-line ri-xl !text-[#16A085]"></i>,
    description: "Automate policy deployment with CI/CD",
  },
  {
    type: "link",
    label: "Custom Data Loading",
    svgIcon: <i className="ri-database-2-line ri-xl !text-[#16A085]"></i>,
    description: "Load external data into your access logic",
  },
  {
    type: "link",
    label: "Auditing & Logs",
    svgIcon: <i className="ri-file-list-line ri-xl !text-[#16A085]"></i>,
    description: "Track and monitor permission changes",
  },
];
export const socialsRow = [
  {
    type: "link",
    href: "https://permit-io.slack.com/join/shared_invite/zt-nz6yjgnp-RlP9rtOPwO0n0aH_vLbmBQ#/shared-invite/email",
    label: "Permit.io Slack Community",
    svgIcon: <i className="ri-slack-line ri-xl !text-[#846358]"></i>,
  },
  {
    type: "link",
    href: "https://github.com/permitio",
    label: "Github",
    svgIcon: <i className="ri-github-line ri-xl !text-[#846358]"></i>,
  },
];
