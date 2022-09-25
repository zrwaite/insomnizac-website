# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# This file is the source Rails uses to define your schema when running `bin/rails
# db:schema:load`. When creating a new database, `bin/rails db:schema:load` tends to
# be faster and is potentially less error prone than running all of your
# migrations from scratch. Old migrations may fail to apply correctly if those
# migrations use external dependencies or application code.
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema[7.0].define(version: 2022_09_25_192527) do
  create_table "projects", id: :bigint, default: -> { "unique_rowid()" }, force: :cascade do |t|
    t.string "name", null: false
    t.string "slug", null: false
    t.string "github_name", null: false
    t.string "devpost_link"
    t.string "project_link"
    t.datetime "created_at", precision: nil, default: -> { "now()" }, null: false
    t.datetime "updated_at", precision: nil, default: -> { "now()" }, null: false
    t.string "image"
    t.boolean "featured", default: false
    t.string "additional_languages", default: [], array: true
  end

  create_table "users", id: :bigint, default: -> { "unique_rowid()" }, force: :cascade do |t|
    t.string "email"
    t.string "password_hash"
    t.boolean "confirmed", default: false
    t.datetime "created_at", precision: nil, null: false
    t.datetime "updated_at", precision: nil, null: false
  end

end
