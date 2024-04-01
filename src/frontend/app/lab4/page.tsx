"use client";
import {
  JSXElementConstructor,
  Key,
  PromiseLikeOfReactNode,
  ReactElement,
  ReactNode,
  ReactPortal,
  useState,
} from "react";
import { Button } from "@nextui-org/react";
import ApproximationChartComponent from "@/components/ApproximationChart";
import Script from "next/script";

const saveObjectToFile = (
  object: { key: string; anotherKey: string },
  filename: string
) => {
  const json = JSON.stringify(object);
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};
const LinearEquationPage = () => {
  const [solution, setSolution] = useState<any>("");
  const [isOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");
  const [value, setValue] = useState<string>("");
  const [solutionFile, setSolutionFile] = useState<any>("");

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
      function: "",
      coefficients: [],
      differences: [],
      epsilon_values: [],
      pearson_correlation: 0,
      phi_values: [],
      data_points: [],
      standard_deviation: 0,
    });

    const lines = value.trim().split("\n");
    const x = lines[0]?.split(" ").map(Number); // Convert string array to number array
    const y = lines[1]?.split(" ").map(Number); // Convert string array to number array

    console.log(JSON.stringify({ x, y }));

    try {
      const response = await fetch(
        "http://localhost:8000/approximation/string",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ x, y }),
        }
      );

      const data = await response.json();
      console.log(data.result);

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }

      setSolution(data.result);
      setSolutionFile(data.result);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  };

  const handleSubmitFile = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    setSolution({
      err: "",
      function: "",
      coefficients: [],
      differences: [],
      epsilon_values: [],
      pearson_correlation: 0,
      phi_values: [],
      data_points: [],
      standard_deviation: 0,
    });

    try {
      const formData = new FormData(event.currentTarget);
      const response = await fetch("http://127.0.0.1:8000/approximation/file", {
        method: "POST",
        body: formData,
      });
      const data = await response.json();

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      
      setSolution(data.result);
      setSolutionFile(data.result);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while uploading: ${error}`);
    }
  };

  const handleFileSave = () => {
    if (solutionFile) {
      saveObjectToFile(solutionFile, "solution.json");
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
            «Аппроксимация функции методом наименьших квадратов»
          </p>

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitString}>
                <textarea
                  name="data"
                  placeholder="
                1.2 2.9 4.1 5.5 6.7 7.8 9.2 10.3\n
                7.4 9.5 11.1 12.9 14.6 17.3 18.2 20.7
                "
                  rows={3}
                  value={value}
                  onChange={(e) => setValue(e.target.value)}
                  className="block w-full rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ввод параметров из файла
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitFile} encType="multipart/form-data">
                <input
                  name="file"
                  type="file"
                  className="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <Button
                    type="submit"
                    className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Рассчитать
                  </Button>
                </div>
              </form>
            </div>
          </div>

          <div className="response-data border-t border-gray-900/10 pb-12">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 mt-5">
              Решение
            </h1>
            <div className="mt-2 flex items-center justify-start gap-x-6">
              <Button
                onClick={handleFileSave}
                className="rounded-md bg-indigo-600 px-3 py-2 text-md font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
              >
                Скачать
              </Button>
            </div>
            <div className="mt-10">
              <div className="py-2">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  Coefficients
                </label>
                {solution &&
                  solution?.coefficients.map(
                    (
                      coefficient: string | number | null | undefined,
                      index: number
                    ) => (
                      <div key={index} className="px-2 py-1">
                        <span className="px-2 block w-full rounded-md bg-gray-100 py-1.5 text-gray-900 sm:text-sm sm:leading-6">
                          Coefficient {index + 1}: {coefficient}
                        </span>
                      </div>
                    )
                  )}
              </div>
            </div>

            <div className="mt-10">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Differences
              </label>
              <div className="py-2">
                {solution &&
                  solution?.differences.map(
                    (
                      difference: string | number | null | undefined,
                      index: number
                    ) => (
                      <div key={index} className="px-2 py-1">
                        <span className="px-2 block w-full rounded-md bg-gray-100 py-1.5 text-gray-900 sm:text-sm sm:leading-6">
                          Difference {index + 1}: {difference}
                        </span>
                      </div>
                    )
                  )}
              </div>
            </div>

            <div className="mt-10">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Epsilon Values
              </label>
              <div className="py-2">
                {solution &&
                  solution?.epsilon_values.map(
                    (
                      epsilon: string | number | null | undefined,
                      index: number
                    ) => (
                      <div key={index} className="px-2 py-1">
                        <span className="px-2 block w-full rounded-md bg-gray-100 py-1.5 text-gray-900 sm:text-sm sm:leading-6">
                          Epsilon {index + 1}: {epsilon}
                        </span>
                      </div>
                    )
                  )}
              </div>
            </div>

            <div className="mt-10">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Function
              </label>
              <div className="py-2 px-2 w-full rounded-md bg-gray-100 text-gray-900 sm:text-sm sm:leading-6">
                <input
                  type="text"
                  name="standardDeviation"
                  id="standardDeviation"
                  value={solution?.function ? solution.function : ""}
                  disabled
                  className="px-2 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div className="mt-10">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Pearson Correlation
              </label>
              <div className="py-2 px-2 w-full rounded-md bg-gray-100 text-gray-900 sm:text-sm sm:leading-6">
                <input
                  type="text"
                  name="pearsonCorrelation"
                  id="pearsonCorrelation"
                  value={
                    solution?.pearson_correlation
                      ? solution.pearson_correlation
                      : 0
                  }
                  disabled
                  className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div className="mt-10">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Standard Deviation
              </label>
              <div className="py-2 px-2 w-full rounded-md bg-gray-100 text-gray-900 sm:text-sm sm:leading-6">
                <input
                  type="text"
                  name="standardDeviation"
                  id="standardDeviation"
                  value={
                    solution?.standard_deviation
                      ? solution.standard_deviation
                      : 0
                  }
                  disabled
                  className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>
          </div>

          <div id="gd" className="sm:col-span-6 col-span-1"></div>
          <ApproximationChartComponent solution={solution} />
        </div>
      </div>
    </>
  );
};

export default LinearEquationPage;
