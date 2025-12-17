import { load } from "cheerio";
import benchmark from "htmlparser-benchmark";

export default function cheerio() {
  return new Promise((res) => {
    var bench = benchmark(function (html, callback) {
      const $ = load(html, { xml: true });
      callback();
    });

    bench.on("progress", function (key) {
      // console.log('finished parsing ' + key + '.html');
    });

    bench.on("result", function (stat) {
      console.log(
        "cheerio-htmlparser2  :" +
          stat.mean().toPrecision(6) +
          " ms/file ± " +
          stat.sd().toPrecision(6),
      );
      res();
    });
  });
}
