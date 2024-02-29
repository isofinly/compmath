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
import { useState } from "react";
import {
  Table,
  TableHeader,
  TableColumn,
  TableBody,
  TableRow,
  TableCell,
  getKeyValue,
} from "@nextui-org/react";
const LinearEquationPage = () => {
  const [solution, setSolution] = useState<any>("");
  const [value, setValue] = useState<string>("");
  const [isOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");
  const [rows, setRows] = useState(2);

  const options = {
    scales: {
      y: {
        title: {
          display: true,
          text: solution
            ? `Вектор погрешности 10^-${getMaxZeros(solution)}`
            : "Вектор погрешности",
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
        text: "График функции",
      },
    },
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

  const handleOpen = () => {
    setIsOpen(true);
  };

  const handleClose = () => {
    setIsOpen(false);
  };

  ChartJS.register(LinearScale, PointElement, LineElement, Tooltip, Legend);

  const handleSubmitString = async (
    event: React.FormEvent<HTMLFormElement>
  ) => {
    event.preventDefault();

    setSolution({
      sol: [],
      acc: [],
      c: [],
      mtrx: [],
      err: "",
      iter: "",
    });
    try {
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

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      setSolution(data);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  };

  const handleSubmitFile = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    setSolution({
      sol: [],
      acc: [],
      c: [],
      mtrx: [],
      err: "",
      iter: "",
    });
    try {
      const formData = new FormData(event.currentTarget);
      const response = await fetch(
        "http://127.0.0.1:8000/linear_equation/file",
        {
          method: "POST",
          body: formData,
        }
      );
      const data = await response.json();

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      setSolution(data);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while uploading: ${error}`);
    }
  };

  function getMaxZeros(solution: any) {
    const max = Math.max(...solution.acc);
    const zeros = Math.abs(Math.floor(Math.log10(max))) - 1;
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

  const halfdivisonTableRows = [
    {
      key: "1",
      number: 1,
      a: 0.123,
      b: 0.123,
      x: 0.123,
      f_a: 0.123,
      f_b: 0.123,
      f_x: 0.123,
      abs_delta: 0.123,
    },
  ];

  const halfdivisonTableColumns = [
    {
      key: "number",
      label: "№ шага",
    },
    {
      key: "a",
      label: "a",
    },
    {
      key: "b",
      label: "b",
    },
    {
      key: "x",
      label: "x",
    },
    {
      key: "f_a",
      label: "f(a)",
    },
    {
      key: "f_b",
      label: "f(b)",
    },
    {
      key: "f_x",
      label: "f(x)",
    },
    {
      key: "abs_delta",
      label: "|a-b|",
    },
  ];

  const newtonTableRows = [
    {
      key: "1",
      number: 1,
      xk: 0.123,
      f_xk: 0.123,
      f_prime_xk: 0.123,
      xk_plus_one: 0.123,
      abs_delta_xk: 0.123,
    },
  ];

  const newtonTableColumns = [
    {
      label: "№ шага",
      key: "number",
    },
    {
      label: "x_k",
      key: "xk",
    },
    {
      label: "f(x_k)",
      key: "f_xk",
    },
    {
      label: "f'(x_k)",
      key: "f_prime_xk",
    },
    {
      label: "x_k+1",
      key: "xk_plus_one",
    },
    {
      label: "|x_k-x_{k+1}|",
      key: "abs_delta_xk",
    },
  ];

  const simpleiterationTableRows = [
    {
      key: "1",
      number: 1,
      xk: 0.123,
      f_xk: 0.123,
      f_xk_plus_one: 0.123,
      phi_xk: 0.123,
      abs_delta_xk: 0.123,
    },
  ];

  const simpleiterationTableColumns = [
    {
      label: "№ шага",
      key: "number",
    },
    {
      label: "x_k",
      key: "xk",
    },
    {
      label: "f(x_k)",
      key: "f_xk",
    },
    {
      label: "x_k+1",
      key: "f_xk_plus_one",
    },
    {
      label: "phi(x_k)",
      key: "phi_xk",
    },
    {
      label: "|x_k-x_{k+1}|",
      key: "abs_delta_xk",
    },
  ];

  return (
    <>
      <div className="container mx-auto space-y-12 py-8">
        <div className="border-gray-900/10 border-b-2 pb-12">
          <h1 className="text-3xl font-semibold leading-7 text-gray-900">
            Лабораторная работа
          </h1>
          <p className="mt-1 text-md leading-6 text-gray-600 mb-6">
            «Численное решение нелинейных уравнений и систем нелинейных
            уравнений»
          </p>

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitString}>
                <textarea
                  name="data"
                  placeholder="1 8 \n
                  0,001
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
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitFile} encType="multipart/form-data">
                <input
                  name="file"
                  type="file"
                  className="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <button
                    type="submit"
                    className="rounded-md bg-indigo-600 px-3 py-2 text-md font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Рассчитать
                  </button>
                </div>
              </form>
            </div>
          </div>

          <div className="border-t border-gray-900/10 pb-12">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 mt-5">
              Решение
            </h1>
            {solution && solution.err && (
              <div
                className="mt-10 grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4"
                role="alert"
              >
                <p className="font-bold">Внимание!</p>
                <p>{solution.err}</p>
              </div>
            )}
            <div className="mt-10 grid grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center">
              <div className="col-span-2 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
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
                <label className="block text-md font-medium leading-6 text-gray-900">
                  x
                </label>
                <div className="py-2">
                  <div className="col-span-1 w-fit py-1">
                    <input
                      type="text"
                      name="iter"
                      id="iter"
                      value={"aboba"}
                      disabled
                      className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    />
                  </div>
                </div>
              </div>
              <div className="col-span-1 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  f(x)
                </label>
                <div className="py-2">
                  <div className="col-span-1 w-fit py-1">
                    <input
                      type="text"
                      name="iter"
                      id="iter"
                      value={"aboba"}
                      disabled
                      className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    />
                  </div>
                </div>
              </div>

              <div className="col-span-2 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  Уточнение корня уравнения методом половинного деления
                </label>
                <div className="py-2">
                  <Table aria-label="Example table with dynamic content">
                    <TableHeader columns={halfdivisonTableColumns}>
                      {(column) => (
                        <TableColumn key={column.key}>
                          {column.label}
                        </TableColumn>
                      )}
                    </TableHeader>
                    <TableBody items={halfdivisonTableRows}>
                      {(item) => (
                        <TableRow key={item.key}>
                          {(columnKey) => (
                            <TableCell>
                              {getKeyValue(item, columnKey)}
                            </TableCell>
                          )}
                        </TableRow>
                      )}
                    </TableBody>
                  </Table>
                </div>
              </div>

              <div className="col-span-2 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  Уточнение корня уравнения методом Ньютона
                </label>
                <div className="py-2">
                  <Table aria-label="Example table with dynamic content">
                    <TableHeader columns={newtonTableColumns}>
                      {(column) => (
                        <TableColumn key={column.key}>
                          {column.label}
                        </TableColumn>
                      )}
                    </TableHeader>
                    <TableBody items={newtonTableRows}>
                      {(item) => (
                        <TableRow key={item.key}>
                          {(columnKey) => (
                            <TableCell>
                              {getKeyValue(item, columnKey)}
                            </TableCell>
                          )}
                        </TableRow>
                      )}
                    </TableBody>
                  </Table>
                </div>
              </div>

              <div className="col-span-2 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  Уточнение корня уравнения методом простой итерации
                </label>
                <div className="py-2">
                  <Table aria-label="Example table with dynamic content">
                    <TableHeader columns={simpleiterationTableColumns}>
                      {(column) => (
                        <TableColumn key={column.key}>
                          {column.label}
                        </TableColumn>
                      )}
                    </TableHeader>
                    <TableBody items={simpleiterationTableRows}>
                      {(item) => (
                        <TableRow key={item.key}>
                          {(columnKey) => (
                            <TableCell>
                              {getKeyValue(item, columnKey)}
                            </TableCell>
                          )}
                        </TableRow>
                      )}
                    </TableBody>
                  </Table>
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
