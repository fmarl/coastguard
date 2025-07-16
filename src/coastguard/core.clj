(ns coastguard.core
  (:gen-class)
  (:require
   [clojure.string :as str]
   [clojure.tools.cli :as cli]
   [coastguard.dns :refer [hostname-exists?]]
   [coastguard.homoglyphs :refer [mutate-using-homoglyphs]]
   [coastguard.tlds :refer [mutate-using-tlds]]))

(defn deconstruct-endpoint [endpoint]
  (str/split endpoint #"\."))

(defn flatten-endpoint-mutations [mutations]
  (if (empty? mutations)
    [""]
    (for [head (first mutations)
          tail (flatten-endpoint-mutations (rest mutations))]
      (str head "." tail))))

(defn homoglyph-mutator [endpoint]
  (let [deconstructed-endpoint (deconstruct-endpoint endpoint)]
    (map mutate-using-homoglyphs (butlast deconstructed-endpoint))))

(defn tlds-mutator [_]
  (mutate-using-tlds))

(defn compose-mutators [mutators endpoint]
  (apply concat (map #(% endpoint) mutators)))

(defn filter-existing-hostnames [endpoint-mutations]
  (pmap hostname-exists? endpoint-mutations))

(defn filter-valid-endpoints [checked-endpoints]
  (filter (fn [x]
            (true? (last x))) checked-endpoints))

(defn run [endpoint]
  (println (str "Mutating " endpoint))
  (let [valid-endpoints (filter-valid-endpoints
                         (filter-existing-hostnames
                          (flatten-endpoint-mutations
                           (compose-mutators [homoglyph-mutator tlds-mutator] endpoint))))]
    (run! #(println (first %)) valid-endpoints)))

(def cli-options
  [["-e" "--endpoint ENDPOINT" "Which endpoint to mutate"]
   ["-h" "--help"]])

(defn -main
  "Running that program!"
  [& args]
  (let [{:keys [options _ errors summary]} (cli/parse-opts args cli-options)]
    (if (seq errors)
      (println (str "Error: " errors))
      (do
        (when (:help options) (println summary))
        (when (:endpoint options) (run (:endpoint options)))))))