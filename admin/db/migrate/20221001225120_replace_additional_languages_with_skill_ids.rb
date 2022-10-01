class ReplaceAdditionalLanguagesWithSkillIds < ActiveRecord::Migration[7.0]
  def change
    add_column :projects, :skills, :bigint, array: true, default: []
    remove_column :projects, :additional_languages
  end
end
