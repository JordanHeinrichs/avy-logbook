import type {
  CalendarRangeProps,
  CalendarMonthProps,
  CalendarDateProps,
} from "cally";

type MapEvents<T> = {
  [K in keyof T as K extends `on${infer E}` ? `on:${Lowercase<E>}` : K]: T[K];
};

declare module "svelte/elements" {
  interface SvelteHTMLElements {
    "calendar-range": HTMLAttributes<HTMLElement> &
      MapEvents<CalendarRangeProps>;
    "calendar-month": HTMLAttributes<HTMLElement> &
      MapEvents<CalendarMonthProps>;
    "calendar-date": HTMLAttributes<HTMLElement> & MapEvents<CalendarDateProps>;
  }
  interface SVGAttributes<T extends EventTarget> {
    slot?: string | null;
  }
}
