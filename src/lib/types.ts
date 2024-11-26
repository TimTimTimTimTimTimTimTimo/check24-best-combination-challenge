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

export type Combination = {
  package_ids: number[];
  offers: Offer[];
  total_price: number;
};
