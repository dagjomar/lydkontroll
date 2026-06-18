import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test } from "vitest";
import { Shell } from "./Shell";

test("reveals scaffold status when the operator asks for it", async () => {
  const user = userEvent.setup();

  render(<Shell />);

  expect(
    screen.queryByText("Appskall aktivt", { exact: true }),
  ).not.toBeInTheDocument();

  await user.click(
    screen.getByRole("button", {
      name: "Vis systemstatus",
    }),
  );

  expect(
    screen.getByText("Appskall aktivt", { exact: true }),
  ).toBeInTheDocument();
  expect(
    screen.getByRole("button", {
      name: "Skjul systemstatus",
    }),
  ).toHaveAttribute("aria-expanded", "true");
});
