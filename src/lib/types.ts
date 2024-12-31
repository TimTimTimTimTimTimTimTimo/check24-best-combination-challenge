import type { Temporal } from "@js-temporal/polyfill";
import type { Team, Tournament } from "./generated_types";

export type Game = {
  id: number;
  team_home: Team;
  team_away: Team;
  starts_at: Date;
  tournament: Tournament;
};

export type Offer = {
  game_id: number;
  package_id: number;
  live: boolean;
  highlights: boolean;
};

export type Package = {
  id: number;
  name: string;
  monthly_price_cents: number | null;
  monthly_price_yearly_subscription_in_cents: number;
};

export type Coverages = {
  high_coverage: number;
  live_coverage: number;
  some_coverage: number;
  full_coverage: number;
};

export type CombinationsResult = {
  game_count: number;
  orphan_count: number;
  cheapest_combination: Combination;
  smallest_combination?: Combination;
  single_combinations: Combination[];
};

export type CombinationsQuery = {
  team_ids: number[];
  tournament_ids: number[];
  timespan: { start: Temporal.PlainDate; end: Temporal.PlainDate };
  cover_type: CoverType;
};

export type CoverType = "High" | "Live" | "Some" | "Full";

export type Combination = {
  package_ids: number[];
  coverages: Coverages;
  yearly_price_per_month_cents: number;
};
