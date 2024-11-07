import type { Team, Tournament } from "./enums";

export type Game = {
  team_home: Team;
  team_away: Team;
  starts_at: Date;
  tournament: Tournament;
};

export type Offer = {
  game_index: number;
  package_index: number;
  live: boolean;
  highlights: boolean;
};

export type Package = {
  name: string;
  monthly_price_cents: number | null;
  monthly_price_yearly_subscription_in_cents: number;
};
