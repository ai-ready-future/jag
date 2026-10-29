---
type: Product
title: jag
description: A command-line search tool for coding agents that returns only the text
  lines that answer a question, within a size limit.
status: draft
created: '2026-10-05'
updated: '2026-10-29'
lenses:
  problem:
    status: complete
    updated: '2026-10-05'
  personas:
    status: complete
    updated: '2026-10-05'
  journeys:
    status: complete
    updated: '2026-10-05'
  capabilities:
    status: complete
    updated: '2026-10-29'
  scope:
    status: complete
    updated: '2026-10-26'
  success-metrics:
    status: complete
    updated: '2026-10-06'
---

# jag

A command-line search tool for coding agents that returns only the text lines that answer a question, within a size limit.

## Problem

Keyword search returns too many matches, and an agent or a person must judge each one by hand. jag adds that judgment: a term finds the matches, and a plain-language prompt ranks them. The flood costs the people who build agents in tokens, and the people who use agents in time. Agents cope today with raw keyword search, a sub-agent that reads the files, or vector search. Cheap scoring models and agents that use command-line tools make now the right time. jag targets a niche audience of people who build agents. See [PROBLEM-01](problems/PROBLEM-01.md) to [PROBLEM-05](problems/PROBLEM-05.md).

## Personas

jag is a general tool, like grep, and agents are the primary use case. The primary persona is the agent builder. Three more personas use jag from the shell: the operator, the knowledge seeker, and the shell power user. Each persona has a job, a pain, and a wanted outcome. Some users can't send their text to an outside model service; [QUESTION-01](questions/QUESTION-01.md) holds that question, and the Scope lens answers it. See [PERSONA-01](personas/PERSONA-01.md) to [PERSONA-04](personas/PERSONA-04.md) and [NEED-01](needs/NEED-01.md) to [NEED-12](needs/NEED-12.md).

## Journeys

[JOURNEY-01](journeys/JOURNEY-01.md) follows the agent builder from a wrong answer to correct answers over many runs: set up a key, try one command, give jag to the agent, and run the failed question again. [JOURNEY-02](journeys/JOURNEY-02.md) is one shared journey for the operator, the knowledge seeker, and the shell power user: a term search returns too many matches, they add a question to the same command, and the top lines answer it. Two new pains of the agent builder are in [NEED-13](needs/NEED-13.md) and [NEED-14](needs/NEED-14.md).

## Capabilities

jag has 14 capabilities: eight must-be, three performance, and three delighters. See [CAPABILITY-01](capabilities/CAPABILITY-01.md) to [CAPABILITY-14](capabilities/CAPABILITY-14.md). [DECISION-01](decisions/DECISION-01.md) records that jag fails clearly when the model is not available. Simulations of CAPABILITY-01 to CAPABILITY-08 added [DECISION-03](decisions/DECISION-03.md) to [DECISION-46](decisions/DECISION-46.md), [QUESTION-03](questions/QUESTION-03.md), and [CONSTRAINT-04](constraints/CONSTRAINT-04.md) to [CONSTRAINT-06](constraints/CONSTRAINT-06.md). [CAPABILITY-11](capabilities/CAPABILITY-11.md) gives the agent instructions in two forms: a skill that the agent builder installs with one README command ([CONSTRAINT-08](constraints/CONSTRAINT-08.md)), and a snippet to copy. [DECISION-53](decisions/DECISION-53.md) records that the skill is not an editor plugin. [CAPABILITY-13](capabilities/CAPABILITY-13.md) lets the user score with a Clef or Clef-Flash server that the user runs, through the base URL setting and with no key. jag makes no quality claim and has no speed target for Clef. [CAPABILITY-14](capabilities/CAPABILITY-14.md) holds back each snippet that matches a private pattern, on each model service, unless the user turns the check off. [DECISION-58](decisions/DECISION-58.md) to [DECISION-68](decisions/DECISION-68.md) hold the details. A simulation of these two capabilities added [DECISION-71](decisions/DECISION-71.md) to [DECISION-81](decisions/DECISION-81.md) and [CONSTRAINT-10](constraints/CONSTRAINT-10.md), and answered [QUESTION-04](questions/QUESTION-04.md) to [QUESTION-07](questions/QUESTION-07.md).

## Scope

[SCOPE-01](scope/SCOPE-01.md) is release 1, the first public release. It must include the six must-be capabilities, should include agent instructions to copy, and could include the cost, the data flow, and the measure on your own questions. jag scores each search again, so a free repeated search is not in release 1. The release after release 1 ([SCOPE-04](scope/SCOPE-04.md)) adds the local model option: a Clef server that the user runs, and the private-pattern check. Release 2 adds trust and cost, release 3 adds a question without a term, and a later release adds a run with no package install. jag builds no index, writes no answers, has no GUI, and has no hosted service. Release 1 goes public only after the held-out score confirms the gain over grep. [DECISION-69](decisions/DECISION-69.md) replaces [DECISION-02](decisions/DECISION-02.md): the release after release 1 serves users whose text must stay on the machine.

## Success metrics

The north star is answer quality: jag must score at least 0.70 ndcg@10 on the held-out set at release 1 ([METRIC-01](metrics/METRIC-01.md)). Three KPIs track context saved, search speed, and time to first result. Four counter-metrics guard them: cost per search, answer still found, false confidence, and setup failures. The cost cap of $0.02 is an assumption, and [QUESTION-02](questions/QUESTION-02.md) holds the open target for setup failures. See [METRIC-01](metrics/METRIC-01.md) to [METRIC-08](metrics/METRIC-08.md).
