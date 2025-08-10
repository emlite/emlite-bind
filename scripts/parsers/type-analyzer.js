import { flat } from "../utils.js";

/**
 * Extract all type references from a member (attribute, operation, field, etc.)
 * @param {Object} member - The WebIDL member object
 * @returns {Set<string>} Set of referenced type names
 */
export function refNames(member) {
  const out = new Set();
  const scan = (t) => {
    if (!t) return;

    if (Array.isArray(t)) return t.forEach(scan);

    if (typeof t === "object" && t.generic && t.idlType) {
      if (Array.isArray(t.idlType)) t.idlType.forEach(scan);
      else scan(t.idlType);
    }

    const n = flat(t).n;
    if (
      /^(undefined|any|object|Promise|DOMString|USVString|ByteString|CSSOMString|boolean)$/.test(
        n
      ) ||
      /long|short|float|double/.test(n) ||
      n.startsWith("__")
    )
      return;

    out.add(n);

    if (typeof t === "object" && t.idlType) scan(t.idlType);
  };

  if (member.type === "attribute") {
    scan(member.idlType);
  } else if (member.type === "operation") {
    scan(member.idlType);
    member.arguments.forEach((a) => scan(a.idlType));
  } else if (member.type === "constructor") {
    scan(member.idlType);
    if (member.arguments) {
      member.arguments.forEach((a) => scan(a.idlType));
    }
  } else if (member.type === "field") {
    scan(member.idlType);
  }
  return out;
}

/**
 * Get all type references from a collection of members
 * @param {Array} members - Array of WebIDL member objects
 * @returns {Set<string>} Set of all referenced type names
 */
export function getAllTypeRefs(members) {
  const refs = new Set();
  members.forEach((m) => {
    refNames(m).forEach((ref) => refs.add(ref));
  });
  return refs;
}