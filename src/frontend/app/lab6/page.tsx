"use client";
import { useState } from "react";
import Script from "next/script";
import { Input } from "@nextui-org/react";
import SingleChartComponent from "@/components/CustomDifferentialChart";
import DifferentialSolverResults from "@/components/Table";

const Page = () => {
  const [solution, setSolution] = useState<any>("");
  const [isOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");

  const [functionId, setFunctionId] = useState("");
  const [y0, setY0] = useState("");
  const [start, setStart] = useState("");
  const [end, setEnd] = useState("");
  const [h, setH] = useState("");
  const [errorValue, setErrorValue] = useState("");

  const handleOpen = () => {
    setIsOpen(true);
  };

  const handleClose = () => {
    setIsOpen(false);
  };

  const handleSubmitString = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setSolution({
      err: "",
      interpolated_value: 0,
    });

    try {
      const response = await fetch(
        "http://localhost:8000/differentials/string",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            equation_id: Number(functionId),
            y0: Number(y0),
            start: Number(start),
            end: Number(end),
            h: Number(h),
            error: Number(errorValue),
          }),
        }
      );

      const data = await response.json();
      console.log(data.result);

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      setError("");
      setSolution(data.result);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  }

  return (
    <>
      <Script
        src="https://www.desmos.com/api/v1.8/calculator.js?apiKey=dcb31709b452b1cf9dc26972add0fda6"
        onError={(e: Error) => {
          console.error("Script failed to load", e);
        }}
      />
      <div className="container mx-auto space-y-12 py-8">
        <div className="border-gray-900/10 border-b-2 pb-12">
          <h1 className="text-3xl font-semibold leading-7 text-gray-900">
            Лабораторная работа
          </h1>
          <p className="mt-1 text-md leading-6 text-gray-600 mb-6">
            «Решение ОДУ»
          </p>

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitString}>
                <label className="block mt-2 text-md font-medium leading-6 text-gray-900">
                  Выберите ОДУ
                  <br />
                  {"0 => -2.0 * y + x.powi(2)"}
                  <br />
                  {"1 => x.powi(3)-2.0*y"}
                  <br />
                  {"2 => y * x.cos()"}
                  <br />
                </label>
                <Input
                  className="mt-2 flex items-center justify-end gap-x-6"
                  type="number"
                  value={functionId}
                  min={0}
                  max={2}
                  onChange={(e) => setFunctionId((e.target.value))}
                  label="ОДУ"
                  variant="flat"
                />
                <Input
                  className="mt-2 flex items-center justify-end gap-x-6"
                  type="number"
                  value={y0}
                  onChange={(e) => setY0((e.target.value))}
                  label="y_0"
                  variant="flat"
                />
                <Input
                  className="mt-2 flex items-center justify-end gap-x-6"
                  type="number"
                  value={start}
                  onChange={(e) => setStart((e.target.value))}
                  label="x_0"
                  variant="flat"
                />
                <Input
                  className="mt-2 flex items-center justify-end gap-x-6"
                  type="number"
                  variant="flat"
                  label="x_n"
                  value={end}
                  onChange={(e) => setEnd((e.target.value))}
                />
                <Input
                  className="mt-2 flex items-center justify-end gap-x-6"
                  type="number"
                  variant="flat"
                  label="h"
                  value={h}
                  onChange={(e) => setH((e.target.value))}
                />
                <Input
                  className="mt-2 flex items-center justify-end gap-x-6"
                  variant="flat"
                  type="number"
                  label="epsilon"
                  value={errorValue}
                  onChange={(e) => setErrorValue((e.target.value))}
                />

                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <button
                    type="submit"
                    className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Рассчитать
                  </button>
                </div>
              </form>
            </div>
          </div>

          {isOpen && (
            <div className="fixed inset-0 flex items-center justify-center z-50">
              <div className="bg-gray-800 bg-opacity-75 absolute inset-0"></div>
              <div className="relative bg-white p-8 rounded-lg shadow-lg">
                <button
                  className="absolute top-0 right-0  m-2"
                  onClick={handleClose}
                >
                  <svg
                    className="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth="2"
                      d="M6 18L18 6M6 6l12 12"
                    ></path>
                  </svg>
                </button>
                <p>{error}</p>
              </div>
            </div>
          )}
          <div className="response-data border-t border-gray-900/10 pb-12">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 mt-5">
              Решение
            </h1>

            <div>
            <DifferentialSolverResults solution={solution} error={error}/>
            </div>
          </div>

          {solution && (
              <>
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <SingleChartComponent solutionData={solution.Euler} />
                </div>
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <SingleChartComponent solutionData={solution.ExtendedEuler} />
                </div>
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <SingleChartComponent solutionData={solution.Milne} />
                </div>
              </>
            )}
        </div>
      </div>
    </>
  );
};

export default Page;
