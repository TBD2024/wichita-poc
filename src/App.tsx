("use client");

import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import {
  Dialog,
  DialogBackdrop,
  DialogPanel,
  TransitionChild,
} from "@headlessui/react";
import {
  Bars3Icon,
  HomeIcon,
  XMarkIcon,
  Cog6ToothIcon,
  PencilSquareIcon,
} from "@heroicons/react/24/outline";
import { Button } from "./components/button";
import { Field, Label } from "./components/fieldset";
import { Textarea } from "./components/textarea";
import { Select } from "./components/select";
import {
  DescriptionDetails,
  DescriptionList,
  DescriptionTerm,
} from "./components/description-list";

import "./App.css";

const navigation = [
  { name: "Home", href: "#", slug: "home", icon: HomeIcon, current: false },
  {
    name: "Post-Processing",
    href: "#",
    slug: "post-processing",
    icon: PencilSquareIcon,
    current: false,
  },
];

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function App() {
  const [page, setPage] = useState("home");
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [prompt, setPrompt] = useState("");
  const [promptCache, setPromptCache] = useState<string[]>([]);
  const [enhancedPrompt, setEnhancedPrompt] = useState("");

  async function enhancePrompt() {
    setSubmitting(true);
    if (enhancedPrompt !== "") {
      setPromptCache(promptCache.concat(enhancedPrompt));
    }
    setEnhancedPrompt(await invoke("prompt_enhancer", { prompt }));
    setSubmitting(false);
  }

  return (
    <>
      <div>
        <Dialog
          open={sidebarOpen}
          onClose={setSidebarOpen}
          className="relative z-50 lg:hidden"
        >
          <DialogBackdrop
            transition
            className="fixed inset-0 bg-gray-900/80 transition-opacity duration-300 ease-linear data-[closed]:opacity-0"
          />

          <div className="fixed inset-0 flex">
            <DialogPanel
              transition
              className="relative mr-16 flex w-full max-w-xs flex-1 transform transition duration-300 ease-in-out data-[closed]:-translate-x-full"
            >
              <TransitionChild>
                <div className="absolute left-full top-0 flex w-16 justify-center pt-5 duration-300 ease-in-out data-[closed]:opacity-0">
                  <button
                    type="button"
                    onClick={() => setSidebarOpen(false)}
                    className="-m-2.5 p-2.5"
                  >
                    <span className="sr-only">Close sidebar</span>
                    <XMarkIcon
                      aria-hidden="true"
                      className="h-6 w-6 text-white"
                    />
                  </button>
                </div>
              </TransitionChild>
              {/* Sidebar component, swap this element with another sidebar if you like */}
              <div className="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-2 ring-1 ring-white/10">
                <div className="flex h-16 shrink-0 items-center">
                  <img
                    alt="Psyborg AI"
                    src="psyborg_logo2.png"
                    className="h-8 w-auto"
                  />
                </div>
                <nav className="flex flex-1 flex-col">
                  <ul role="list" className="flex flex-1 flex-col gap-y-7">
                    <li>
                      <ul role="list" className="-mx-2 space-y-1">
                        {navigation.map((item) => (
                          <li key={item.name}>
                            <a
                              href={item.href}
                              className={classNames(
                                item.current
                                  ? "bg-gray-800 text-white"
                                  : "text-gray-400 hover:bg-gray-800 hover:text-white",
                                "group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6"
                              )}
                              onClick={() => {
                                setPage(item.slug);
                                setSidebarOpen(false);
                              }}
                            >
                              <item.icon
                                aria-hidden="true"
                                className="h-6 w-6 shrink-0"
                              />
                              {item.name}
                            </a>
                          </li>
                        ))}
                      </ul>
                    </li>
                  </ul>
                </nav>
              </div>
            </DialogPanel>
          </div>
        </Dialog>

        {/* Static sidebar for desktop */}
        <div className="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col">
          {/* Sidebar component, swap this element with another sidebar if you like */}
          <div className="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6">
            <div className="flex shrink-0 items-center">
              <img
                alt="Your Company"
                src="psyborg_logo2.png"
                className="h-15 w-auto"
              />
            </div>
            <nav className="flex flex-1 flex-col">
              <ul role="list" className="flex flex-1 flex-col gap-y-7">
                <li>
                  <ul role="list" className="-mx-2 space-y-1">
                    {navigation.map((item) => (
                      <li key={item.name}>
                        <a
                          href={item.href}
                          className={classNames(
                            item.current
                              ? "bg-gray-800 text-white"
                              : "text-gray-400 hover:bg-gray-800 hover:text-white",
                            "group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6"
                          )}
                          onClick={() => setPage(item.slug)}
                        >
                          <item.icon
                            aria-hidden="true"
                            className="h-6 w-6 shrink-0"
                          />
                          {item.name}
                        </a>
                      </li>
                    ))}
                  </ul>
                </li>
                <li className="mt-auto">
                  <a
                    href="#"
                    className="group -mx-2 flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-400 hover:bg-gray-800 group-hover:text-white"
                  >
                    <Cog6ToothIcon
                      aria-hidden="true"
                      className="h-6 w-6 shrink-0 text-gray-400 group-hover:text-white"
                    />
                    Settings
                  </a>
                </li>
              </ul>
            </nav>
          </div>
        </div>

        <div className="sticky top-0 z-40 flex items-center gap-x-6 bg-gray-900 px-4 py-4 shadow-sm sm:px-6 lg:hidden">
          <button
            type="button"
            onClick={() => setSidebarOpen(true)}
            className="-m-2.5 p-2.5 text-gray-400 lg:hidden"
          >
            <span className="sr-only">Open sidebar</span>
            <Bars3Icon aria-hidden="true" className="h-6 w-6" />
          </button>
          <div className="flex-1 text-sm font-semibold leading-6 text-white">
            Home
          </div>
          <a href="#">
            <span className="sr-only">Your profile</span>
            <img
              alt=""
              src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
              className="h-8 w-8 rounded-full bg-gray-800"
            />
          </a>
        </div>

        <main className="py-10 lg:pl-72">
          <div className="px-4 sm:px-6 lg:px-8">
            {/* PAGES */}

            {/* HOME */}
            {page === "home" && (
              <>
                <div className="divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow text-black">
                  <div className="px-4 py-5 sm:px-6">Prompt Enhancer</div>
                  <form
                    className="text-black"
                    onSubmit={(e) => {
                      e.preventDefault();
                      enhancePrompt();
                    }}
                  >
                    <div className="px-4 py-5 sm:p-6 text-black">
                      <Field>
                        <Label>Prompt</Label>
                        <Textarea
                          className="text-black border border-black rounded-md"
                          name="prompt"
                          onChange={(e) => setPrompt(e.target.value)}
                        />
                      </Field>
                    </div>
                    <div className="px-4 py-4 sm:px-6">
                      <Button
                        className="cursor-pointer"
                        type="submit"
                        disabled={submitting}
                      >
                        {submitting ? (
                          <Cog6ToothIcon className="w-6 h-6 animate-spin text-white" />
                        ) : (
                          "Enhance Prompt"
                        )}
                      </Button>
                    </div>
                  </form>
                </div>

                {/* Display enhanced prompt */}
                <div className="mt-6 text-black">
                  <h2 className="text-lg font-semibold">Enhanced Prompt:</h2>
                  <p className="mt-2 text-gray-500">{enhancedPrompt}</p>
                </div>
              </>
            )}

            {/* POST-PROCESSING */}
            {page === "post-processing" && (
              <div className="divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow text-black">
                <div className="px-4 py-5 sm:px-6">Post-Processing</div>
                <div className="px-4 py-5 sm:p-6 text-black">
                  <DescriptionList>
                    <DescriptionTerm>Model</DescriptionTerm>
                    <DescriptionDetails>
                      <Field>
                        <Select name="model">
                          <option value="stable-diffusion">
                            Stable Diffusion
                          </option>
                          <option value="juggernaut">Juggernaut</option>
                          <option value="dreamshaperv8">DreamShaper v8</option>
                        </Select>
                      </Field>
                    </DescriptionDetails>

                    <DescriptionTerm>LoRAs</DescriptionTerm>
                    <DescriptionDetails>
                      <Field>
                        <Select name="lora">
                          <option value="detail">
                            Detail Tweaker
                          </option>
                          <option value="color-enhancer">
                            Color Enhancer
                          </option>
                          <option value="noise-reduction">
                            Noise Reduction
                          </option>
                        </Select>
                      </Field>
                    </DescriptionDetails>
                  </DescriptionList>
                </div>
              </div>
            )}
          </div>
        </main>
      </div>
    </>
  );
}
