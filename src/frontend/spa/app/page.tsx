"use client";
import {
  Chart as ChartJS,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
} from "chart.js";
import { Scatter } from "react-chartjs-2";
import { Key, useState } from "react";

const LinearEquationPage = () => {
  const [solution, setSolution] = useState<string>("");
  const [value, setValue] = useState<string>("");
  ChartJS.register(LinearScale, PointElement, LineElement, Tooltip, Legend);

  const handleSubmitString = async (
    event: React.FormEvent<HTMLFormElement>
  ) => {
    event.preventDefault();

    const response = await fetch(
      "http://127.0.0.1:8000/linear_equation/string",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          data: value,
        }),
      }
    );
    const data = await response.json();

    setSolution(data);
  };

  const handleSubmitFile = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const response = await fetch("http://127.0.0.1:8000/linear_equation/file", {
      method: "POST",
      body: formData,
    });
    const data = await response.json();

    setSolution(data);
  };

  const options = {
    scales: {
      y: {
        title: {
          display: true,
          text: "Вектор погрешности",
        },
      },
      x: {
        title: {
          display: true,
          text: "Вектор неизвестных х",
        },
      },
    },
    plugins: {
      legend: {
        position: "top" as const,
      },
      title: {
        display: true,
        text: "График функции"
      }
    }
  };

  const data = {
    datasets: [
      {
        label: "Вектора",
        data: solution && convertSolutionToPlotData(solution),
        backgroundColor: "rgba(255, 99, 132, 1)",
      },
    ],
  };

  function getMaxZeros(solution: any) {
    const max = Math.max(...solution.acc);
    const zeros = Math.abs(Math.floor(Math.log10(max)))-1;
    return zeros;
  }

  function convertSolutionToPlotData(solution: any) {
    const data = [];
    if (!solution) return;
    for (let i = 0; i < solution.sol.length; i++) {
      data.push({
        x: solution.sol[i],
        y: solution.acc[i] * 10 ** getMaxZeros(solution),
        // y: solution.acc[i],
      });
    }
    return data;
  }

  return (
    <>
      <div className="container mx-auto space-y-12 py-8">
        <div className="border-gray-900/10 border-b-2 pb-12">
          <h1 className="text-xl font-semibold leading-7 text-gray-900">
            Лабораторная работа
          </h1>
          <p className="mt-1 text-sm leading-6 text-gray-600 mb-6">
            «Решение системы линейных алгебраических уравнений СЛАУ»
          </p>

          <div className="col-span-full mb-6">
            <label className="block text-sm font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitString}>
                <textarea
                  name="data"
                  rows={3}
                  value={value}
                  onChange={(e) => setValue(e.target.value)}
                  className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <div className="mt-6 flex items-center justify-end gap-x-6">
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
          <div className="col-span-full mb-6">
            <label className="block text-sm font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitFile} encType="multipart/form-data">
                <input
                  name="file"
                  type="file"
                  rows={3}
                  className="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <div className="mt-6 flex items-center justify-end gap-x-6">
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

          <div className="border-t border-gray-900/10 pb-12">
            <h1 className="text-xl font-semibold leading-7 text-gray-900 mt-5">
              Решение
            </h1>

            <div className="mt-10 grid grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center">
              <div className="col-span-2 w-full">
                <label className="block text-sm font-medium leading-6 text-gray-900">
                  Количество итераций
                </label>
                <div className="py-2">
                  <input
                    type="text"
                    name="iter"
                    id="iter"
                    value={solution ? solution.iter : ""}
                    disabled
                    className="px-2 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  />
                </div>
              </div>

              <div className="col-span-1 w-full">
                <label className="block text-sm font-medium leading-6 text-gray-900">
                  Вектор неизвестных
                </label>
                <div className="py-2">
                  {solution &&
                    solution.sol.map((item: Key | null | undefined) => (
                      <div className="col-span-1 w-fit py-1" key={item}>
                        <input
                          type="text"
                          name="iter"
                          id="iter"
                          value={item}
                          disabled
                          className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        />
                      </div>
                    ))}
                </div>
              </div>

              <div className="col-span-1 w-full">
                <label className="block text-sm font-medium leading-6 text-gray-900">
                  Вектор погрешности
                </label>
                <div className="py-2">
                  {solution &&
                    solution.acc.map((item: Key | null | undefined) => (
                      <div className="col-span-1 w-fit py-1" key={item}>
                        <input
                          type="text"
                          name="iter"
                          id="iter"
                          value={item}
                          disabled
                          className=" px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        />
                      </div>
                    ))}
                </div>
              </div>

              <div className="col-span-2 w-full">
                <Scatter options={options} data={data} />
              </div>
            </div>
          </div>

          <div id="gd" className="sm:col-span-6 col-span-1"></div>
        </div>
      </div>
    </>
  );
};

export default LinearEquationPage;
