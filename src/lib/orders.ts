import { writable } from "svelte/store";

// Load initial data from localStorage
const storedOrders = JSON.parse(localStorage.getItem("orders") || "[]");

// Create a writable store
export const orders =
  writable<{ id: number; items: any[]; status: string }[]>(storedOrders);

// Subscribe to the store and save changes to localStorage
orders.subscribe((value) => {
  localStorage.setItem("orders", JSON.stringify(value));
});

// Listen for changes in localStorage (e.g., from other windows)
window.addEventListener("storage", (event) => {
  if (event.key === "orders") {
    const updatedOrders = JSON.parse(event.newValue || "[]");
    orders.set(updatedOrders); // Update the store with the new data
  }
});
