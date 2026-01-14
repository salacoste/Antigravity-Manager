---
description: Adapt BMad commands to the Antigravity workflow: Product (Epics/Stories) -> Dev -> QA Gate -> Product Validation -> Merge.
---

# Antigravity Product Lifecycle Workflow

This workflow orchestrates the entire lifecycle of a feature in the Antigravity project, using BMad agents to enforce quality and process steps.

## Phase 0: Prerequisites & Setup
**Goal**: Ensure the repository is ready for the lifecycle.

1.  **Required Directories**:
    - `docs/stories/`: For Epic and Story files.
    - `docs/architecture/`: For system design documents.
2.  **Required Documents**:
    - `docs/project-brief.md`: The high-level product vision.
    - `docs/architecture/*.md`: At least one architecture overview.


## Phase 1: Product Definition & Validation
**Role**: Product Owner / PM
**Goal**: Create or update Epics and Stories, then validate them.

1.  **Generate/Update Epics & Stories**:
    - Run: `/po` (or `/bmad-bmm-workflows-create-epics-and-stories`)
    - *Action*: Follow the agent's prompts to analyze the PRD/Architecture and generate stories.
2.  **Validation**:
    - Review the generated files in `docs/stories/`.
    - Ensure stories have clear Acceptance Criteria.
    - *Decision*: If changes are needed, manually edit or ask the agent to refine.

## Phase 2: Implementation (The Cycle)
**Role**: Developer
**Goal**: Implement a single Story.

1.  **Select a Story**:
    - Pick a story from `docs/stories/` that is ready for dev.
2.  **Start Development**:
    - Run: `/dev` (or `/bmad-bmm-workflows-dev-story`)
    - *Action*: The agent will:
        - Create a feature branch.
        - Implement the code.
        - Write and pass tests.
        - Mark the story as `IMPLEMENTED` in the story file.
3.  **Handoff to QA**:
    - Once the `dev-story` workflow completes successfully, the story is ready for the Quality Gate.

## Phase 3: QA & Quality Gate
**Role**: QA Engineer
**Goal**: Validate the implementation and generate a Quality Gate decision.

1.  **Run Quality Trace**:
    - Run: `/qa` (or `/bmad-bmm-workflows-testarch-trace`)
    - *Action*: This acts as the **Gate File** generator. It traces requirements to tests and outputs a coverage report.
2.  **Validate Gate**:
    - Review the generated Trace Matrix.
    - If the result is **PASS**:
        - Update the Story status to `QA_PASSED`.
    - If the result is **FAIL** or **CONCERNS**:
        - Return to **Phase 2** for fixes.

## Phase 4: Final Product Validation & Merge
**Role**: Product Owner
**Goal**: Final sign-off and merge to main.

1.  **Product Verification**:
    - PM reviews the implemented story (functionally) and the QA Gate result.
2.  **Completion**:
    - If approved:
        - Update Story status to `COMPLETED`.
    - If rejected:
        - Return to **Phase 2** (or Phase 1 if requirements were wrong).
3.  **Merge**:
    - *Action*: Merge the feature branch into the main branch.
    - *Loop*: Pick the next story and repeat from **Phase 2**.
