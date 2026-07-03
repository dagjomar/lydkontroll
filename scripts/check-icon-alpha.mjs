import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { fileURLToPath } from "node:url";
import { inflateSync } from "node:zlib";

const iconDirectory = new URL("../src-tauri/icons/", import.meta.url);
const pngNames = [
  "32x32.png",
  "64x64.png",
  "128x128.png",
  "128x128@2x.png",
  "icon.png",
];

function paeth(left, above, upperLeft) {
  const estimate = left + above - upperLeft;
  const leftDistance = Math.abs(estimate - left);
  const aboveDistance = Math.abs(estimate - above);
  const upperLeftDistance = Math.abs(estimate - upperLeft);
  return leftDistance <= aboveDistance && leftDistance <= upperLeftDistance
    ? left
    : aboveDistance <= upperLeftDistance
      ? above
      : upperLeft;
}

function readCornerAlpha(path) {
  const png = readFileSync(path);
  if (png.toString("hex", 0, 8) !== "89504e470d0a1a0a") {
    throw new Error(`${path} is not a PNG`);
  }

  const idat = [];
  let width;
  let height;
  let colorType;
  for (let offset = 8; offset < png.length; ) {
    const length = png.readUInt32BE(offset);
    const type = png.toString("ascii", offset + 4, offset + 8);
    const data = png.subarray(offset + 8, offset + 8 + length);
    if (type === "IHDR") {
      width = data.readUInt32BE(0);
      height = data.readUInt32BE(4);
      const bitDepth = data[8];
      colorType = data[9];
      if (bitDepth !== 8 || ![4, 6].includes(colorType) || data[12] !== 0) {
        throw new Error(
          `${path} must be a non-interlaced 8-bit PNG with alpha`,
        );
      }
    } else if (type === "IDAT") {
      idat.push(data);
    } else if (type === "IEND") {
      break;
    }
    offset += length + 12;
  }

  const bytesPerPixel = colorType === 6 ? 4 : 2;
  const stride = width * bytesPerPixel;
  const encoded = inflateSync(Buffer.concat(idat));
  const pixels = Buffer.alloc(stride * height);
  let inputOffset = 0;
  for (let row = 0; row < height; row += 1) {
    const filter = encoded[inputOffset++];
    for (let column = 0; column < stride; column += 1) {
      const value = encoded[inputOffset++];
      const outputOffset = row * stride + column;
      const left =
        column >= bytesPerPixel ? pixels[outputOffset - bytesPerPixel] : 0;
      const above = row > 0 ? pixels[outputOffset - stride] : 0;
      const upperLeft =
        row > 0 && column >= bytesPerPixel
          ? pixels[outputOffset - stride - bytesPerPixel]
          : 0;
      const predictor = [
        0,
        left,
        above,
        Math.floor((left + above) / 2),
        paeth(left, above, upperLeft),
      ][filter];
      if (predictor === undefined)
        throw new Error(`${path} uses unknown PNG filter ${filter}`);
      pixels[outputOffset] = (value + predictor) & 0xff;
    }
  }

  const alphaAt = (x, y) =>
    pixels[y * stride + x * bytesPerPixel + bytesPerPixel - 1];
  return [
    alphaAt(0, 0),
    alphaAt(width - 1, 0),
    alphaAt(0, height - 1),
    alphaAt(width - 1, height - 1),
  ];
}

function assertTransparentCorners(path) {
  const alpha = readCornerAlpha(path);
  const displayPath = path instanceof URL ? fileURLToPath(path) : path;
  if (alpha.some((value) => value !== 0)) {
    throw new Error(
      `${basename(displayPath)} has opaque corner alpha: ${alpha.join(", ")}`,
    );
  }
  console.log(
    `transparent corners: ${basename(displayPath)} (${alpha.join(", ")})`,
  );
}

for (const name of pngNames)
  assertTransparentCorners(new URL(name, iconDirectory));

const temporaryDirectory = mkdtempSync(join(tmpdir(), "lydkontroll-icon-"));
const iconset = join(temporaryDirectory, "icon.iconset");
try {
  execFileSync("iconutil", [
    "-c",
    "iconset",
    fileURLToPath(new URL("icon.icns", iconDirectory)),
    "-o",
    iconset,
  ]);
  for (const name of [
    "icon_16x16.png",
    "icon_16x16@2x.png",
    "icon_32x32.png",
    "icon_32x32@2x.png",
    "icon_128x128.png",
    "icon_128x128@2x.png",
    "icon_256x256.png",
    "icon_256x256@2x.png",
    "icon_512x512.png",
    "icon_512x512@2x.png",
  ]) {
    assertTransparentCorners(join(iconset, name));
  }
} finally {
  rmSync(temporaryDirectory, { recursive: true, force: true });
}
